use bevy::render::view::Msaa;
use bevy_map_camera::MapCamera;
use bevy_mesh_outline::OutlineCamera;

use crate::components::main_camera::MainCamera;

pub fn spawn_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((MapCamera, MainCamera, Msaa::Off, OutlineCamera));
}
