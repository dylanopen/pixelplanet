use bevy::app::{App, Startup};

use crate::spawn::{configure_camera_controls, spawn_main_camera, spawn_money_display, spawn_selector_mesh, spawn_terrain_voxels};

pub fn register_startups(app: &mut App) {
    app.add_systems(Startup, spawn_main_camera);
    app.add_systems(Startup, configure_camera_controls);
    app.add_systems(Startup, spawn_money_display);
    app.add_systems(Startup, spawn_selector_mesh);
    app.add_systems(Startup, spawn_terrain_voxels);

}
