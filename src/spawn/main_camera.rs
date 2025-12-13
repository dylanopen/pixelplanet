pub fn spawn_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((MapCamera, MainCamera, Msaa::Off, OutlineCamera));
}

