use bevy::{ecs::system::Commands, light::AmbientLight, log::info};

pub fn spawn_ambient_light(mut commands: Commands) {
    info!("Spawning ambient light");
    commands.insert_resource(AmbientLight {
        color: bevy::color::Color::WHITE,
        brightness: 100.0,
        affects_lightmapped_meshes: false,
    });
}
