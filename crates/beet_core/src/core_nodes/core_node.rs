use super::*;
use beet_ecs::action_list;
use beet_ecs::prelude::*;
use bevy_math::Vec3;


// for now we need to manually keep in sync with crates/beet_ecs/src/builtin_nodes/builtin_nodes.rs
action_list!(CoreNode, [
	//core
	Translate,
	//steer
	Seek,
	Wander,
	SetVelocity,
	FindSteerTarget,
	DespawnSteerTarget,
	ScoreSteerTarget,
	SucceedOnArrive,
	//ecs
	ConstantScore,
	EmptyAction,
	FallbackSelector,
	Repeat,
	SetRunResult,
	SucceedInDuration,
	SequenceSelector,
	UtilitySelector
]);


pub fn translate_graph() -> BehaviorGraph<CoreNode> {
	BehaviorTree::new(Translate::new(Vec3::new(0., 1., 0.))).into()
}
