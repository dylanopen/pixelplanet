use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::Res,
};

use crate::{
    components::ToolType,
    messages::{ClickTileMessage, SellTileMessage},
    resources::{ActiveToolType, Tilemap},
};

pub fn use_delete_tile(
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    mut sell_tile_mw: MessageWriter<SellTileMessage>,
    active_tool_type: Res<ActiveToolType>,
    tilemap: Res<Tilemap>,
) {
    let tool_type = &active_tool_type.0;
    let ToolType::DeleteTile = tool_type else {
        return;
    };
    for msg in click_tile_mr.read() {
        let pos = msg.pos;
        if tilemap.is_empty(pos) {
            continue;
        }

        sell_tile_mw.write(SellTileMessage { pos });
    }
}
