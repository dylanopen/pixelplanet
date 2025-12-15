use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    light::NotShadowCaster,
    math::primitives::Cuboid,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

use crate::components::tile_selected_indicator::TileSelectedIndicator;

pub fn spawn_selector_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(0.5, -100.0, 0.5);
    commands.spawn((
        TileSelectedIndicator,
        Mesh3d(meshes.add(Cuboid::new(1.02, 1.02, 1.02))),
        MeshMaterial3d(materials.add(Color::linear_rgba(1.0, 1.0, 1.0, 0.25))),
        transform,
        NotShadowCaster,
    ));
}
