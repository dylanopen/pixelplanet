use bevy::app::{App, Update};

use crate::executors;

pub fn register_executors(app: &mut App) {
    app.add_systems(Update, executors::update_current_hovered_tile);
    app.add_systems(Update, executors::update_tile_selected_indicator);
    app.add_systems(Update, executors::set_money_res);
}
