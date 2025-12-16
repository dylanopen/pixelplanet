use bevy::ecs::message::{MessageReader, MessageWriter};

use crate::{
    components::{RoadVariant, TileType},
    messages::{ClickTileMessage, PurchaseTileMessage},
};

pub fn place_road(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut purchase_tile_mw: MessageWriter<PurchaseTileMessage>,
) {
    for msg in click_tile_mr.read() {
        let tiletype = TileType::Road(RoadVariant(0));
        let pos = msg.pos;

        purchase_tile_mw.write(PurchaseTileMessage { pos, tiletype });
    }
}
