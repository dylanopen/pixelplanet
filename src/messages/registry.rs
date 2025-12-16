use bevy::app::App;

use crate::messages;

pub fn register_messages(app: &mut App) {
    app.add_message::<messages::ClickTileMessage>();
    app.add_message::<messages::CursorTileHoverMessage>();
    app.add_message::<messages::UpdateTileMessage>();
    app.add_message::<messages::UpdateTileModelMessage>();
    app.add_message::<messages::SetTileMessage>();
    app.add_message::<messages::SetMoneyMessage>();
}
