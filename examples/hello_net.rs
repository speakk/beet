use beet_examples::prelude::*;
use bevy::prelude::*;

pub fn main() {
	App::new()
		.add_plugins(ExamplePluginFull)
		.add_systems(
			Startup,
			(
				scenes::beet_debug,
				scenes::camera_2d,
				scenes::ui_terminal_input,
				scenes::hello_net,
			),
		)
		.run();
}

/*
STDOUT:

Started: Sentence Selector
Started: Attack Behavior

*/
