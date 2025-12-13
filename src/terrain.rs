use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::primitives::{Cuboid, Plane3d},
    mesh::{Mesh, Mesh3d, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
    utils::default,
};
use rand::{Rng, rng};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain_voxels);
        app.add_systems(Startup, spawn_ground_plane);
    }
}

