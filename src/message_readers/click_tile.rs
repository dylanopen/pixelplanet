use bevy::{
    ecs::{
        message::{MessageReader, MessageWriter},
    },
};

use crate::{components::{RoadVariant, Tile, TileType}, messages::{ClickTileMessage, SetTileMessage}};

pub fn place_road(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut set_tile_mw: MessageWriter<SetTileMessage>,
) {
    for msg in click_tile_mr.read() {
        let tile = Tile {
            tiletype: TileType::Road(RoadVariant::StraightNS),
            entity: None,
        };
        let pos = msg.pos;

        set_tile_mw.write(SetTileMessage {
            pos, tile 
        });
    }
}
