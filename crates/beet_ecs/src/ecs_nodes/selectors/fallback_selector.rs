use super::*;
use crate::prelude::*;
use bevy::prelude::*;

/// An action that runs all of its children in order until one succeeds.
///
/// Logical OR: `RUN child1 OTHERWISE child2 etc`
///
/// If a child succeeds it will succeed.
///
/// If the last child fails it will fail.
#[derive(Default)]
#[derive_action(child_components=[Score])]
pub struct FallbackSelector;
fn fallback_selector(
	mut commands: Commands,
	selectors: Query<(Entity, &FallbackSelector, &Edges), With<Running>>,
	children_running: Query<(), With<Running>>,
	children_results: Query<&RunResult>,
) {
	for (parent, _selector, children) in selectors.iter() {
		if any_child_running(children, &children_running) {
			continue;
		}

		match first_child_result(children, &children_results) {
			Some((index, result)) => match result {
				&RunResult::Success => {
					commands.entity(parent).insert(RunResult::Success);
				}
				&RunResult::Failure => {
					if index == children.len() - 1 {
						commands.entity(parent).insert(RunResult::Failure);
					} else {
						commands.entity(children[index + 1]).insert(Running);
					}
				}
			},
			None => {
				commands.entity(children[0]).insert(Running);
			}
		}
	}
}
