use bevy::app::{App, Startup};

use crate::spawn::{configure_camera_controls, spawn_main_camera};

pub fn register_startups(app: &mut App) {
    app.add_systems(Startup, spawn_main_camera);
    app.add_systems(Startup, configure_camera_controls);

}
