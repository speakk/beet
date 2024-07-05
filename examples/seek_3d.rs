use beet_examples::prelude::*;
use bevy::prelude::*;

pub fn main() {
	App::new()
		.add_plugins(ExamplePluginFull)
		.add_systems(
			Startup,
			(
				scenes::beet_debug,
				scenes::lighting_3d,
				scenes::ground_3d,
				scenes::seek_3d,
			),
		)
		.run();
}
