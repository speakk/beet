use crate::prelude::*;
use anyhow::Result;
use beet_net::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Resource)]
pub struct DespawnEntityHandler {
	pub send: Publisher<DespawnEntityPayload>,
	pub recv: Subscriber<DespawnEntityPayload>,
}

impl TopicHandler<DespawnEntityPayload> for DespawnEntityHandler {
	fn topic() -> Topic {
		Topic::new(ENTITY_TOPIC, TopicScheme::PubSub, TopicMethod::Delete)
	}
}

impl DespawnEntityHandler {
	pub fn new(relay: &mut Relay) -> Result<Self> {
		Ok(Self {
			send: Self::publisher(relay)?,
			recv: Self::subscriber(relay)?,
		})
	}
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DespawnEntityPayload {
	/// If None, despawn all entities
	pub beet_id: Option<BeetEntityId>,
}

impl DespawnEntityPayload {
	pub fn all() -> Self { Self { beet_id: None } }
	pub fn new(beet_id: BeetEntityId) -> Self {
		Self {
			beet_id: Some(beet_id),
		}
	}
}

pub fn handle_despawn_entity(
	mut commands: Commands,
	entity_map: ResMut<BeetEntityMap>,
	mut handler: ResMut<DespawnEntityHandler>,
) -> Result<()> {
	for msg in handler.recv.try_recv_all()? {
		if let Some(beet_id) = msg.beet_id {
			if let Ok(entity) = entity_map.get(beet_id) {
				commands.get_entity(*entity).map(|mut e| e.despawn());
			}
		} else {
			for entity in entity_map.map().values() {
				commands.get_entity(*entity).map(|mut e| e.despawn());
			}
		}
	}

	Ok(())
}
