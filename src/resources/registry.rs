use bevy::app::App;

use crate::resources;

pub fn init_resources(app: &mut App) {
    app.init_resource::<resources::current_hovered_tile::CurrentHoveredTile>();
}
