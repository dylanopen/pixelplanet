use bevy::{
    app::{Plugin, Startup},
    camera::Camera3d,
    ecs::component::Component,
    render::view::Msaa,
};
use bevy_map_camera::MapCamera;
use bevy_mesh_outline::OutlineCamera;

use crate::spawn::main_camera::spawn_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

