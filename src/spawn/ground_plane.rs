pub fn spawn_ground_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Transform::from_xyz(0.5, 1.0, 0.5),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(64., 64.))),
        Ground,
    ));
}

