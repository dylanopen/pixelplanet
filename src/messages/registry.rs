use bevy::app::App;

use crate::messages;

pub fn register_messages(app: &mut App) {
    app.add_message::<messages::click_tile::ClickTileMessage>();
    app.add_message::<messages::cursor_tile_hover::CursorTileHoverMessage>();
}
