use bevy::app::{App, Update};

use crate::forwarders;

pub fn register_forwarders(app: &mut App) {
    app.add_systems(Update, forwarders::place_tool_tile);
    app.add_systems(Update, forwarders::handle_purchase_tile);
    app.add_systems(Update, forwarders::set_added_money);
    app.add_systems(Update, forwarders::change_active_tool);
    app.add_systems(Update, forwarders::use_delete_tile);
    app.add_systems(Update, forwarders::sell_tile);
}
