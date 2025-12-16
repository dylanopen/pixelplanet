use bevy::app::{App, Plugin, Startup};

use crate::spawn::spawn_ambient_light;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
    }
}
