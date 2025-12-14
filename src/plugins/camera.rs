use bevy::app::{App, Plugin, Startup};

use crate::spawn::configure_camera_controls;
use crate::spawn::spawn_main_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_camera);
        app.add_systems(Startup, configure_camera_controls);
    }
}
