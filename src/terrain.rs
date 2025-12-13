use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::primitives::Cuboid,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain_voxels);
    }
}

#[derive(Component)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)]
pub struct TerrainVoxel;

fn spawn_grass_voxel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    commands.spawn((
        TerrainVoxel,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.875, 1.0))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.3, 0.7, 0.3))),
        transform,
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
