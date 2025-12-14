use bevy::app::{Plugin, Startup};

use crate::spawn::main_camera::spawn_main_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_main_camera);
    }
}
