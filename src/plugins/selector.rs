use bevy::app::{App, Plugin, Startup, Update};

use crate::{executors, triggers, spawn::spawn_selector_mesh};

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selector_mesh);

        app.add_systems(Update, triggers::map_cursor_hover::map_cursor_hover);
        app.add_systems(
            Update,
            executors::cursor_tile_hover::update_current_hovered_tile,
        );
        app.add_systems(
            Update,
            executors::cursor_tile_hover::update_tile_selected_indicator,
        );

        app.add_systems(Update, triggers::left_click::on_left_click);
        app.add_systems(Update, executors::click_tile::place_road);
    }
}
