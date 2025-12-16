use bevy::app::{App, Update};

use crate::forwarders;

pub fn register_forwarders(app: &mut App) {
    app.add_systems(Update, forwarders::place_road);
    app.add_systems(Update, forwarders::handle_purchase_tile);
}
