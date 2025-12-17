use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::Res,
};

use crate::{
    components::ToolType,
    messages::{ClickTileMessage, PurchaseTileMessage},
    resources::ActiveToolType,
};

pub fn place_tool_tile(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut purchase_tile_mw: MessageWriter<PurchaseTileMessage>,
    active_tool_type: Res<ActiveToolType>,
) {
    let tool_type = &active_tool_type.0;
    #[expect(
        irrefutable_let_patterns,
        reason = "more tool types will be added later"
    )]
    let ToolType::PlaceTile(tiletype) = tool_type else {
        return;
    };
    for msg in click_tile_mr.read() {
        let pos = msg.pos;

        purchase_tile_mw.write(PurchaseTileMessage {
            pos,
            tiletype: tiletype.clone(),
        });
    }
}
