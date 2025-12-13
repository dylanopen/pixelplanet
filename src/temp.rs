use bevy::{
    asset::{AssetServer, Assets}, color::Color, ecs::system::{Commands, ResMut}, light::EnvironmentMapLight, math::{Vec2, Vec3, primitives::Cuboid}, mesh::{Mesh, Mesh3d}, pbr::{MeshMaterial3d, StandardMaterial}, post_process::bloom::Bloom, scene::SceneRoot, transform::components::Transform, utils::default
};

pub fn spawn_example_road(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    let road_handle = asset_server.load("buildings/road1.vox");

    commands.spawn((
        SceneRoot(road_handle),
        Transform::from_xyz(16.0, 16.0, 0.0).with_scale(Vec3::splat(1.0 / 64.0)),
    ));
}

pub fn spawn_example_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
