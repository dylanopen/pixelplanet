use bevy::{
    app::{Plugin, Startup},
    camera::Camera3d,
    ecs::component::Component,
    render::view::Msaa,
};
use bevy_map_camera::MapCamera;
use bevy_mesh_outline::OutlineCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

fn spawn_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((MapCamera, MainCamera, Msaa::Off, OutlineCamera));
}
