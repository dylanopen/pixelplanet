use bevy::app::{App, Update};

use crate::triggers;

pub fn register_triggers(app: &mut App) {
    app.add_systems(Update, triggers::on_left_click);
    app.add_systems(Update, triggers::map_cursor_hover);
    app.add_systems(Update, triggers::tick_money_increase);
    app.add_systems(Update, triggers::change_toolbar_tool);
    app.add_systems(Update, triggers::tick_resident_moveins);
}
