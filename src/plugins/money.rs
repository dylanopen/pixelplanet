use bevy::app::{App, Plugin, Startup};

use crate::spawn::spawn_money_display;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_money_display);
    }
}
