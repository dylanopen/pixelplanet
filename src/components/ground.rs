use bevy::ecs::component::Component;
use bevy::transform::components::Transform;

#[derive(Component)]
#[require(Transform)]
pub struct Ground;
