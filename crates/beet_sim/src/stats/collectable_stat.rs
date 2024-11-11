use crate::prelude::*;
use bevy::prelude::*;

//// Added to children of Collectables and Zones for consideration in [`FindStatSteerTarget`]
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct StatProvider;


/// Add this as a child of an entity to make it collectable.
/// This will be able to be picked up.
#[derive(Component, Reflect)]
#[reflect(Default, Component)]
#[require(StatProvider)]
pub struct CollectableStat {
	pub radius: f32,
}

impl Default for CollectableStat {
	fn default() -> Self { Self { radius: 0.5 } }
}


pub fn pickup_collectable(
	mut commands: Commands,
	stat_map: Res<StatMap>,
	collectors: Query<&GlobalTransform, With<Collector>>,
	query: Populated<(&GlobalTransform, &CollectableStat, &StatId, &StatValue)>,
) {
	for (transform, collectable_stat, stat_id, stat_value) in query.iter() {
		let rad_sq = collectable_stat.radius * collectable_stat.radius;
		for collector_transform in collectors.iter() {
			if transform
				.translation()
				.distance_squared(collector_transform.translation())
				> rad_sq
			{
				continue;
			}
			todo!("pickup_collectable");
		}
	}
}
