use bevy::{ecs::{message::MessageWriter, system::Res}, input::{ButtonInput, mouse::MouseButton}};

use crate::{messages::ClickTileMessage, resources::CurrentHoveredTile};

pub fn on_left_click(
    current_hovered_tile: Res<CurrentHoveredTile>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut click_tile_mw: MessageWriter<ClickTileMessage>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(pos) = current_hovered_tile.pos {
        click_tile_mw.write(ClickTileMessage { pos });
    }
}
