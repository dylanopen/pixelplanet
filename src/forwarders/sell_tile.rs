use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::Res,
};

use crate::{
    messages::{AddMoneyMessage, ClearTileMessage, SellTileMessage},
    resources::Tilemap,
};

pub fn sell_tile(
    mut sell_tile_mr: MessageReader<SellTileMessage>,
    mut clear_tile_mw: MessageWriter<ClearTileMessage>,
    mut add_money_mw: MessageWriter<AddMoneyMessage>,
    tilemap: Res<Tilemap>,
) {
    for msg in sell_tile_mr.read() {
        let pos = msg.pos;
        let sell_price = match tilemap.get_tile(pos) {
            Some(tile) => tile.tiletype.get_cost() * 0.75,
            None => {
                continue;
            }
        };

        clear_tile_mw.write(ClearTileMessage { pos });

        add_money_mw.write(AddMoneyMessage {
            increase: sell_price,
        });
    }
}
