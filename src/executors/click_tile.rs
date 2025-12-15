use bevy::ecs::message::{MessageReader, MessageWriter};

use crate::{
    components::{RoadVariant, TileType},
    messages::{ClickTileMessage, SetTileMessage},
};

pub fn place_road(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut set_tile_mw: MessageWriter<SetTileMessage>,
) {
    for msg in click_tile_mr.read() {
        let tiletype = TileType::Road(RoadVariant(0));
        let pos = msg.pos;

        set_tile_mw.write(SetTileMessage { pos, tiletype });
    }
}
