//! This module parses the settings.ron file and builds a lightyear configuration from it
#![allow(unused_variables)]
#[cfg(not(target_family = "wasm"))]
use async_compat::Compat;
use bevy::prelude::Resource;
#[cfg(not(target_family = "wasm"))]
use bevy::tasks::IoTaskPool;
use bevy::utils::Duration;
use lightyear::prelude::client;
use lightyear::prelude::client::Authentication;
#[cfg(not(target_family = "wasm"))]
use lightyear::prelude::server;
use lightyear::prelude::CompressionConfig;
use lightyear::prelude::LinkConditionerConfig;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

/// We parse the settings.ron file to read the settings
pub fn settings<T: DeserializeOwned>(settings_str: &str) -> T {
	ron::de::from_str::<T>(settings_str)
		.expect("Could not deserialize the settings file")
}

#[derive(
	Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum ClientTransports {
	#[cfg(not(target_family = "wasm"))]
	Udp,
	WebTransport {
		certificate_digest: String,
	},
	WebSocket,
	#[cfg(not(target_family = "wasm"))]
	Steam {
		app_id: u32,
	},
}

#[derive(
	Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum ServerTransports {
	Udp {
		local_port: u16,
	},
	WebTransport {
		local_port: u16,
	},
	WebSocket {
		local_port: u16,
	},
	#[cfg(not(target_family = "wasm"))]
	Steam {
		app_id: u32,
		server_ip: Ipv4Addr,
		game_port: u16,
		query_port: u16,
	},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conditioner {
	/// One way latency in milliseconds
	pub(crate) latency_ms: u16,
	/// One way jitter in milliseconds
	pub(crate) jitter_ms: u16,
	/// Percentage of packet loss
	pub(crate) packet_loss: f32,
}

impl Conditioner {
	pub fn build(&self) -> LinkConditionerConfig {
		LinkConditionerConfig {
			incoming_latency: Duration::from_millis(self.latency_ms as u64),
			incoming_jitter: Duration::from_millis(self.jitter_ms as u64),
			incoming_loss: self.packet_loss,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSettings {
	/// If true, disable any rendering-related plugins
	pub(crate) headless: bool,

	/// If true, enable bevy_inspector_egui
	pub(crate) inspector: bool,

	/// Possibly add a conditioner to simulate network conditions
	pub(crate) conditioner: Option<Conditioner>,

	/// Which transport to use
	pub transport: Vec<ServerTransports>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientSettings {
	/// The client id
	pub(crate) client_id: u64,

	/// The client port to listen on
	pub(crate) client_port: u16,

	/// The ip address of the server
	pub server_addr: Ipv4Addr,

	/// The port of the server
	pub server_port: u16,

	/// Which transport to use
	pub(crate) transport: ClientTransports,

	/// Possibly add a conditioner to simulate network conditions
	pub(crate) conditioner: Option<Conditioner>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct SharedSettings {
	/// An id to identify the protocol version
	pub protocol_id: u64,

	/// a 32-byte array to authenticate via the Netcode.io protocol
	pub private_key: [u8; 32],

	/// compression options
	pub(crate) compression: CompressionConfig,
}

#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
	pub server: ServerSettings,
	pub client: ClientSettings,
	pub shared: SharedSettings,
}

pub fn build_server_netcode_config(
	conditioner: Option<&Conditioner>,
	shared: &SharedSettings,
	transport_config: server::ServerTransport,
) -> server::NetConfig {
	let conditioner = conditioner.map_or(None, |c| {
		Some(LinkConditionerConfig {
			incoming_latency: Duration::from_millis(c.latency_ms as u64),
			incoming_jitter: Duration::from_millis(c.jitter_ms as u64),
			incoming_loss: c.packet_loss,
		})
	});
	let netcode_config = server::NetcodeConfig::default()
		.with_protocol_id(shared.protocol_id)
		.with_key(shared.private_key);
	let io_config = server::IoConfig {
		transport: transport_config,
		conditioner,
		compression: shared.compression,
	};
	server::NetConfig::Netcode {
		config: netcode_config,
		io: io_config,
	}
}

/// Parse the settings into a list of `NetConfig` that are used to configure how the lightyear server
/// listens for incoming client connections
#[cfg(not(target_family = "wasm"))]
pub fn get_server_net_configs(settings: &Settings) -> Vec<server::NetConfig> {
	settings
		.server
		.transport
		.iter()
		.map(|t| match t {
			ServerTransports::Udp { local_port } => {
				build_server_netcode_config(
					settings.server.conditioner.as_ref(),
					&settings.shared,
					server::ServerTransport::UdpSocket(SocketAddr::new(
						Ipv4Addr::UNSPECIFIED.into(),
						*local_port,
					)),
				)
			}
			ServerTransports::WebTransport { local_port } => {
				// this is async because we need to load the certificate from io
				// we need async_compat because wtransport expects a tokio reactor
				let certificate = IoTaskPool::get()
					.scope(|s| {
						s.spawn(Compat::new(async {
							server::Identity::load_pemfiles(
								"assets/certificates/cert.pem",
								"assets/certificates/key.pem",
							)
							.await
							.unwrap()
						}));
					})
					.pop()
					.unwrap();
				let digest =
					certificate.certificate_chain().as_slice()[0].hash();
				println!(
					"Generated self-signed certificate with digest: {}",
					digest
				);
				build_server_netcode_config(
					settings.server.conditioner.as_ref(),
					&settings.shared,
					server::ServerTransport::WebTransportServer {
						server_addr: SocketAddr::new(
							Ipv4Addr::UNSPECIFIED.into(),
							*local_port,
						),
						certificate,
					},
				)
			}
			ServerTransports::WebSocket { local_port } => {
				build_server_netcode_config(
					settings.server.conditioner.as_ref(),
					&settings.shared,
					server::ServerTransport::WebSocketServer {
						server_addr: SocketAddr::new(
							Ipv4Addr::UNSPECIFIED.into(),
							*local_port,
						),
					},
				)
			}
			ServerTransports::Steam {
				app_id,
				server_ip,
				game_port,
				query_port,
			} => {
				panic!("unsupported")
			}
		})
		.collect()
}

/// Build a netcode config for the client
pub fn build_client_netcode_config(
	client_id: u64,
	server_addr: SocketAddr,
	conditioner: Option<&Conditioner>,
	shared: &SharedSettings,
	transport_config: client::ClientTransport,
) -> client::NetConfig {
	let conditioner = conditioner.map_or(None, |c| Some(c.build()));
	let auth = Authentication::Manual {
		server_addr,
		client_id,
		private_key: shared.private_key,
		protocol_id: shared.protocol_id,
	};
	let netcode_config = client::NetcodeConfig::default();
	let io_config = client::IoConfig {
		transport: transport_config,
		conditioner,
		compression: shared.compression,
	};
	client::NetConfig::Netcode {
		auth,
		config: netcode_config,
		io: io_config,
	}
}

/// Parse the settings into a `NetConfig` that is used to configure how the lightyear client
/// connects to the server
pub fn get_client_net_config(
	settings: &Settings,
	client_id: u64,
) -> client::NetConfig {
	let server_addr = SocketAddr::new(
		settings.client.server_addr.into(),
		settings.client.server_port,
	);
	let client_addr = SocketAddr::new(
		Ipv4Addr::UNSPECIFIED.into(),
		settings.client.client_port,
	);
	match &settings.client.transport {
		#[cfg(not(target_family = "wasm"))]
		ClientTransports::Udp => build_client_netcode_config(
			client_id,
			server_addr,
			settings.client.conditioner.as_ref(),
			&settings.shared,
			client::ClientTransport::UdpSocket(client_addr),
		),
		ClientTransports::WebTransport { certificate_digest } => {
			build_client_netcode_config(
				client_id,
				server_addr,
				settings.client.conditioner.as_ref(),
				&settings.shared,
				client::ClientTransport::WebTransportClient {
					client_addr,
					server_addr,
					#[cfg(target_family = "wasm")]
					certificate_digest: certificate_digest
						.to_string()
						.replace(":", ""),
				},
			)
		}
		ClientTransports::WebSocket => build_client_netcode_config(
			client_id,
			server_addr,
			settings.client.conditioner.as_ref(),
			&settings.shared,
			client::ClientTransport::WebSocketClient { server_addr },
		),
		#[cfg(not(target_family = "wasm"))]
		_ => {
			panic!("unsupported")
		}
	}
}
