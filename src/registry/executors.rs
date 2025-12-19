use bevy::app::{App, Update};

use crate::executors;

pub fn register_executors(app: &mut App) {
    app.add_systems(Update, executors::update_current_hovered_tile);
    app.add_systems(Update, executors::update_tile_selected_indicator);
    app.add_systems(Update, executors::set_money_res);
    app.add_systems(Update, executors::set_tile_in_tilemap);
    app.add_systems(Update, executors::update_money_display);
    app.add_systems(Update, executors::update_tiles);
    app.add_systems(Update, executors::update_tile_models);
    app.add_systems(Update, executors::set_active_tool_type);
    app.add_systems(Update, executors::set_tile_blank);
}
