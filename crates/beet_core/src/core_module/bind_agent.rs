use beet::graph::BeetRoot;
use bevy::prelude::*;



#[derive(Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BindAgentToFirstGraph;




pub fn bind_agent_to_first_graph(
	world: &mut World, // mut commands: Commands,
	                   // roots: Query<Entity, >,
	                   // query: Query<Entity, With<BindAgentToFirstGraph>>,
) {
	for entity in world
		.query_filtered::<Entity, With<BindAgentToFirstGraph>>()
		.iter(world)
		.collect::<Vec<_>>()
		.into_iter()
	{
		let Some(first) = world
			.query_filtered::<Entity, With<BeetRoot>>()
			.iter(world)
			.next()
		else {
			log::warn!("No first graph found to bind agent to");
			continue;
		};

		// let new_node = EntityIdent(first).deep_clone(world).unwrap();
		let _ = first;
		world.entity_mut(entity).remove::<BindAgentToFirstGraph>();
		todo!("rethink this, no more binding");
	}
}
