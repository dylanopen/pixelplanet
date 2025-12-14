use bevy::render::view::Msaa;
use bevy_map_camera::MapCamera;
use bevy_mesh_outline::OutlineCamera;

use crate::components::main_camera::MainCamera;

pub fn spawn_main_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((MainCamera, MapCamera, Msaa::Off, OutlineCamera));
}
