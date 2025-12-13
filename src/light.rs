use std::f32::consts::PI;

use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    light::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, light_consts},
    log::info,
    math::{Quat, Vec3},
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

fn spawn_ambient_light(mut commands: Commands) {
    info!("Spawning ambient light");
    commands.insert_resource(AmbientLight {
        color: bevy::color::Color::WHITE,
        brightness: 100.0,
        affects_lightmapped_meshes: false,
    });
}

fn spawn_sun_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-1000.0, 3000.0, -1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}
