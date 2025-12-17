use bevy::app::{App, Update};

use crate::forwarders;

pub fn register_forwarders(app: &mut App) {
    app.add_systems(Update, forwarders::place_tool_tile);
    app.add_systems(Update, forwarders::handle_purchase_tile);
    app.add_systems(Update, forwarders::set_added_money);
}
