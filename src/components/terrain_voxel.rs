use bevy::ecs::component::Component;
use bevy::mesh::Mesh3d;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::transform::components::Transform;

#[derive(Component)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)]
pub struct TerrainVoxel;
