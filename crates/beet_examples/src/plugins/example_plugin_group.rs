use crate::beet::prelude::*;
use crate::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


/// Kitchen sink plugin, this is all you need for
/// ### Rendering
/// - text
/// - 2d
/// - 3d
/// ### Beet
/// - steering
/// - machine learning
///
pub struct ExamplePluginFull;

impl Plugin for ExamplePluginFull {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			ExampleDefaultPlugins,
			WorldInspectorPlugin::default()
				.run_if(input_toggle_active(false, KeyCode::Tab)),
			DefaultBeetPlugins,
			ExamplePlugins,
		))
		.add_systems(Startup, load_scenes_from_args);
	}
}
/// Just the essentials, no machine learning
pub struct ExamplePluginBasics;

impl Plugin for ExamplePluginBasics {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			ExampleDefaultPlugins,
			DefaultBeetPlugins,
			BeetDebugPluginBase,
			BeetDebugPluginStdout,
			ExampleBasePlugin,
			Example2dPlugin,
			Example3dPlugin,
			UiTerminalPlugin,
			ExampleReplicatePlugin,
			BundlePlaceholderPlugin,
		))
		.add_systems(Startup, load_scenes_from_args);
	}
}

pub struct ExampleBasePlugin;

impl Plugin for ExampleBasePlugin {
	fn build(&self, app: &mut App) {
		app
		.register_type::<Collectable>()
	/*-*/;
	}
}

#[derive(Default)]
pub struct ExamplePlugins;

impl PluginGroup for ExamplePlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(BeetDebugPluginBase)
			.add(BeetDebugPluginStdout)
			.add(ExampleBasePlugin)
			.add(Example2dPlugin)
			.add(Example3dPlugin)
			.add(UiTerminalPlugin)
			.add(ExampleReplicatePlugin)
			.add(BundlePlaceholderPlugin)
			.add(ExampleMlPlugin)
			.add(FrozenLakePlugin)
	}
}


pub struct ExampleMlPlugin;

impl Plugin for ExampleMlPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			// sentence selector
			BertPlugin::default(),
			ActionPlugin::<InsertOnAssetEvent<RunResult, Bert>>::default(),
			AssetPlaceholderPlugin::<Bert>::default(),
			ReadyOnAssetLoadPlugin::<Bert>::default(),
			// qtables (frozen lake)
			AssetPlaceholderPlugin::<QTable<GridPos, GridDirection>>::default(),
			ReadyOnAssetLoadPlugin::<QTable<GridPos, GridDirection>>::default(),
		))
		// fetch
		.add_plugins(ActionPlugin::<(
			InsertOnAssetEvent<RunResult, Bert>,
			FindSentenceSteerTarget<Collectable>,
			RemoveAgentOnRun<Sentence>,
			RemoveAgentOnRun<SteerTarget>,
		)>::default())
			/*-*/;
	}
}



pub struct Example2dPlugin;

impl Plugin for Example2dPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugins(ReadyOnAssetLoadPlugin::<Image>::default())
		.add_systems(Update, follow_cursor_2d)
		// .add_systems(PreUpdate, auto_spawn::auto_spawn.before(PreTickSet))
		.add_systems(Update, randomize_position.in_set(PreTickSet))
		.add_systems(
			Update,
			(update_wrap_around, wrap_around)
			.chain()
			.run_if(|res: Option<Res<WrapAround>>| res.is_some())
			.in_set(PostTickSet),
		)
		.register_type::<AutoSpawn>()
		.register_type::<RandomizePosition>()
		.register_type::<RenderText>()
		.register_type::<WrapAround>()
		.register_type::<FollowCursor2d>()
		/*_*/;
	}
}

pub struct Example3dPlugin;

impl Plugin for Example3dPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			AnimationGraphPlaceholderPlugin,
			AssetPlaceholderPlugin::<AnimationClip>::default(),
			ReadyOnAssetLoadPlugin::<AnimationClip>::default(),
		))
		.add_systems(
			Update,
			(follow_cursor_3d, camera_distance, rotate_collectables),
		)
		.register_type::<FollowCursor3d>()
		.register_type::<CameraDistance>()
		/*-*/;
	}
}
