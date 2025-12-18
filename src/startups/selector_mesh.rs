use bevy::{
    asset::{AssetServer, Assets}, color::Color, ecs::system::{Commands, ResMut}, light::NotShadowCaster, math::{primitives::Cuboid, Vec3}, mesh::{Mesh, Mesh3d}, pbr::{MeshMaterial3d, StandardMaterial}, scene::SceneRoot, transform::components::Transform
};

use crate::{components::tile_selected_indicator::TileSelectedIndicator, consts::MODEL_SIZE};

pub fn spawn_selector_mesh(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(0.5, -100.0, 0.5);
    commands.spawn((
        TileSelectedIndicator,
        SceneRoot(asset_server.load("tiles/road0.vox")),
        MeshMaterial3d(materials.add(Color::linear_rgba(1.0, 1.0, 1.0, 0.25))),
        transform.with_scale(Vec3::splat(1.0) / MODEL_SIZE),
        NotShadowCaster,
    ));
}
