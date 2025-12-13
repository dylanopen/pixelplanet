use bevy::{asset::Assets, ecs::system::{Commands, ResMut}, math::primitives::Plane3d, mesh::{Mesh, Mesh3d, Meshable}, transform::components::Transform};

use crate::components::ground::Ground;

pub fn spawn_ground_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Transform::from_xyz(0.5, 1.0, 0.5),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(64., 64.))),
        Ground,
    ));
}

