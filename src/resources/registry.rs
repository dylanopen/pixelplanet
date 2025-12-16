use bevy::app::App;

use crate::resources;

pub fn init_resources(app: &mut App) {
    app.init_resource::<resources::CurrentHoveredTile>();
    app.init_resource::<resources::Tilemap>();
    app.init_resource::<resources::Money>();
}
