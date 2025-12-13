use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    light::AmbientLight,
    log::info,
};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_ambient_light);
        //app.add_systems(Startup, spawn_sun_light);
    }
}

fn spawn_ambient_light(mut commands: Commands) {
    info!("Spawning ambient light");
    commands.insert_resource(AmbientLight {
        color: bevy::color::Color::WHITE,
        brightness: 800.0,
        affects_lightmapped_meshes: false,
    });
}

