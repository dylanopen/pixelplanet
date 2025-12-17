use bevy::app::{App, Startup};

use crate::startups::*;

pub fn register_startups(app: &mut App) {
    app.add_systems(Startup, spawn_ambient_light);
    app.add_systems(Startup, spawn_main_camera);
    app.add_systems(Startup, configure_camera_controls);
    app.add_systems(Startup, spawn_selector_mesh);
    app.add_systems(Startup, spawn_terrain_voxels);
    app.add_systems(Startup, spawn_ground_plane);

    app.add_systems(Startup, spawn_money_display);
    //app.add_systems(Startup, spawn_build_button);
}
