use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::Res,
};

use crate::{
    components::ToolType,
    messages::{ClickTileMessage, PurchaseTileMessage},
    resources::{ActiveToolType, Tilemap},
};

pub fn place_tool_tile(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut purchase_tile_mw: MessageWriter<PurchaseTileMessage>,
    active_tool_type: Res<ActiveToolType>,
    tilemap: Res<Tilemap>,
) {
    let tool_type = &active_tool_type.0;
    let ToolType::PlaceTile(tiletype) = tool_type else {
        return;
    };
    for msg in click_tile_mr.read() {
        let pos = msg.pos;
        if !tiletype.can_place_at(pos, &tilemap) {
            continue;
        }

        purchase_tile_mw.write(PurchaseTileMessage {
            pos,
            tiletype: tiletype.clone(),
        });
    }
}
