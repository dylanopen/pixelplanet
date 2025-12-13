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
};
use rand::{Rng, rng};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain_voxels);
        app.add_systems(Startup, spawn_ground_plane);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Ground;

#[derive(Component)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)]
pub struct TerrainVoxel;

fn spawn_grass_voxel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    let brightness_noise = rng().random_range(0.7..1.3);
        
    commands.spawn((
        TerrainVoxel,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.3*brightness_noise, 0.7+brightness_noise, 0.3*brightness_noise))),
        transform,
    ));
}

fn spawn_ground_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Transform::from_xyz(0.5, 1.0, 0.5),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(64., 64.))),
        Ground,
    ));
}

fn spawn_terrain_voxels(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -32..32 {
        for z in -32..32 {
            spawn_grass_voxel(
                &mut commands,
                &mut meshes,
                &mut materials,
                Transform::from_xyz(x as f32 + 0.5, 0.5, z as f32 + 0.5),
            );
        }
    }
}
