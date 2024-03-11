use beet_ecs::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_transform::components::Transform;
use serde::Deserialize;
use serde::Serialize;

#[action(system=score_steer_target,components=Score::Fail)]
pub struct ScoreSteerTarget {
	pub radius: f32,
}

impl Default for ScoreSteerTarget {
	fn default() -> Self { Self { radius: 0.5 } }
}

impl ScoreSteerTarget {
	pub fn new(radius: f32) -> Self { Self { radius } }
}

fn score_steer_target(
	transforms: Query<&Transform>,
	agents: Query<(&Transform, &SteerTarget)>,
	mut query: Query<(&TargetAgent, &ScoreSteerTarget, &mut Score)>,
) {
	for (agent, scorer, mut score) in query.iter_mut() {
		if let Ok((transform, target)) = agents.get(**agent) {
			if let Ok(target) = target.position(&transforms) {
				if Vec3::distance(transform.translation, target)
					<= scorer.radius
				{
					*score = Score::Pass;
					continue;
				}
			}
		}
		*score = Score::Fail;
	}
}
// Or<(
// 	Changed<Transform>,
// 	Changed<SteerTarget>,
// 	Changed<ScoreSteerTarget>,
// )>,
