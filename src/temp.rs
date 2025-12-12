use bevy::{asset::Assets, camera::Camera3d, color::Color, ecs::system::{Commands, ResMut}, math::{Vec3, primitives::Cuboid}, mesh::{Mesh, Mesh3d}, pbr::{MeshMaterial3d, StandardMaterial}, transform::components::Transform};

pub fn spawn_example_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

