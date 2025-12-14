use bevy::app::{Plugin, Startup};

use crate::spawn::{ambient_light::spawn_ambient_light, sun_light::spawn_sun_light};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_ambient_light);
        app.add_systems(Startup, spawn_sun_light);
    }
}
