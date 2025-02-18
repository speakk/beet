use crate::beet::prelude::*;
use crate::prelude::*;
use sweet::prelude::*;
use bevy::prelude::*;
use bevyhub::prelude::*;


const SCALE: f32 = 100.;

pub fn flock(mut commands: Commands) {
	let mut rand = RandomSource::default();
	commands.insert_resource(WrapAround::default());

	for _ in 0..300 {
		let position = Vec3::ZERO;
		let position = Vec3::random_in_sphere(&mut rand).with_y(0.) * 500.;
		commands
			.spawn((
				Name::new("Spaceship"),
				BundlePlaceholder::Sprite {
					path: "spaceship_pack/ship_2.png".into(),
					image_mode: default(),
				},
				Transform::from_translation(position)
					.with_scale(Vec3::splat(0.5)),
				RotateToVelocity2d,
				ForceBundle::default(),
				SteerBundle::default().scaled_dist(SCALE),
				VelocityScalar(Vec3::new(1., 1., 0.)),
				GroupSteerAgent,
			))
			.with_children(|agent| {
				agent.spawn((
					Name::new("Flock Behavior"),
					RunOnSpawn,
					TargetEntity(agent.parent_entity()),
					Separate::<GroupSteerAgent>::new(1.).scaled_dist(SCALE),
					Align::<GroupSteerAgent>::new(1.).scaled_dist(SCALE),
					Cohere::<GroupSteerAgent>::new(1.).scaled_dist(SCALE),
					Wander::new(1.).scaled_dist(SCALE),
				));
			});
	}
}
