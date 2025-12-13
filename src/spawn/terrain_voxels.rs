pub fn spawn_terrain_voxels(
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
        transform,
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgb(
                0.3 * brightness_noise,
                0.7 + brightness_noise,
                0.3 * brightness_noise,
            ),
            unlit: true,
            ..default()
        })),
    ));
}
