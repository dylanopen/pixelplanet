use bevy::app::{App, Plugin, Startup};

use crate::spawn::{spawn_ground_plane, spawn_terrain_voxels};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
    }
}
