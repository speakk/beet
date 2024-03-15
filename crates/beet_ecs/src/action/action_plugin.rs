use crate::prelude::*;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PreTickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct TickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct TickSyncSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PostTickSet;


pub struct ActionPlugin<T: ActionSystems, Schedule: ScheduleLabel + Clone> {
	pub schedule: Schedule,
	pub phantom: PhantomData<T>,
}
impl<T: ActionSystems> Default for ActionPlugin<T, Update> {
	fn default() -> Self {
		Self {
			schedule: Update,
			phantom: PhantomData,
		}
	}
}

impl<T: ActionSystems + Send + Sync, Schedule: ScheduleLabel + Clone> Plugin
	for ActionPlugin<T, Schedule>
{
	fn build(&self, app: &mut App) {
		app.configure_sets(self.schedule.clone(), PreTickSet)
			.configure_sets(self.schedule.clone(), TickSet.after(PreTickSet))
			.configure_sets(self.schedule.clone(), TickSyncSet.after(TickSet))
			.configure_sets(
				self.schedule.clone(),
				PostTickSet.after(TickSyncSet),
			)
			.add_systems(
				self.schedule.clone(),
				apply_deferred.after(PreTickSet).before(TickSet),
			)
			.add_systems(
				self.schedule.clone(),
				apply_deferred.after(TickSet).before(TickSyncSet),
			)
			.add_systems(
				self.schedule.clone(),
				apply_deferred.after(TickSyncSet).before(PostTickSet),
			)
			.add_systems(
				self.schedule.clone(),
				update_run_timers
					.run_if(|time: Option<Res<Time>>| time.is_some())
					.in_set(PreTickSet),
			)
			.add_systems(
				self.schedule.clone(),
				cleanup_entity_graph.in_set(PreTickSet),
			)
			.add_systems(
				self.schedule.clone(),
				(sync_running, sync_interrupts).in_set(TickSyncSet),
			);
		T::add_systems(app, self.schedule.clone());
	}
}
