
use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    light::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, light_consts},
    log::info,
    math::Vec3,
    transform::components::Transform,
    utils::default,
};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_ambient_light);
        app.add_systems(Startup, spawn_sun_light);
    }
}

