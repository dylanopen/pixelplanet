use bevy::camera::Camera3d;
use bevy::ecs::component::Component;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

