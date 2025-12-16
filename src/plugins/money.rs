use bevy::app::{App, Plugin, Startup, Update};

use crate::executors::set_money::set_money_res;
use crate::executors::update_money_display::update_money_display;
use crate::spawn::spawn_money_display;
use crate::triggers::tick_money_increase::tick_money_increase;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_money_display);
        app.add_systems(Update, tick_money_increase);
        app.add_systems(Update, set_money_res);
        app.add_systems(Update, update_money_display);
    }
}
