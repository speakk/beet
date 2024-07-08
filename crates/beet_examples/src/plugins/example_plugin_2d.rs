use crate::prelude::*;
use crate::beet::prelude::*;
use bevy::prelude::*;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;
/// Boilerplate for examples
pub struct ExamplePlugin2d;

impl Plugin for ExamplePlugin2d {
	fn build(&self, app: &mut App) {
		app
			// .add_plugins(WorldInspectorPlugin::new())
			.add_plugins(ExampleDefaultPlugins)
			.add_systems(Startup, space_setup)
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
			.insert_resource(WrapAround::default())
		.register_type::<AutoSpawn>()
		.register_type::<RandomizePosition>()
		.register_type::<RenderText>()
		/*-*/;



		let world = app.world_mut();

		world.init_component::<AutoSpawn>();
		world.init_component::<RandomizePosition>();
		world.init_component::<RenderText>();

	}
}


fn space_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// camera
	commands.spawn(Camera2dBundle::default());

	// background
	commands.spawn((
		SpriteBundle {
			texture: asset_server.load("space_background/Space_Stars2.png"),
			transform: Transform::from_translation(Vec3::new(0., 0., -1.))
				.with_scale(Vec3::splat(100.)),
			..default()
		},
		ImageScaleMode::Tiled {
			tile_x: true,
			tile_y: true,
			stretch_value: 0.01,
		},
	));
}
