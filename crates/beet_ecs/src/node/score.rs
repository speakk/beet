use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::cmp::Ordering;
use std::fmt::Debug;

// TODO
/// Indicate this node's parent will use the scores in the next tick.
/// As this is frequently added and removed, it is `SparseSet`.
#[derive(Default, Debug, Component)]
#[component(storage = "SparseSet")]
pub struct Scoring;


/// Used to indicate to selectors how favorable a child node would be to run.
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	Component,
	PartialEq,
	// strum
	strum_macros::Display,
	strum_macros::EnumIter,
	Reflect,
)]
#[reflect(Component)]
pub enum Score {
	#[default]
	/// The node should not run.
	Fail,
	/// The node has a weight, usually in the range `0..1`, where higher is more favorable.
	Weight(f32),
	// Weight(#[number(min = 0, max = 100, step = 1)] u8),
	/// The node should run.
	Pass,
}

impl PartialOrd for Score {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let val = match (self, other) {
			(Score::Fail, Score::Fail) => Ordering::Equal,
			(Score::Fail, _) => Ordering::Less,
			(_, Score::Fail) => Ordering::Greater,
			(Score::Pass, Score::Pass) => Ordering::Equal,
			(Score::Pass, _) => Ordering::Greater,
			(_, Score::Pass) => Ordering::Less,
			(Score::Weight(w1), Score::Weight(w2)) => w1.total_cmp(&w2),
			// (Score::Weight(w1), Score::Weight(w2)) => w1.cmp(&w2),
		};
		Some(val)
	}
}
