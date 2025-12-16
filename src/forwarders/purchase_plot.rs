use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;

use crate::messages::{PurchaseTileMessage, AddMoneyMessage, SetTileMessage};
use crate::resources::Money;

pub fn handle_purchase_tile(
    mut purchase_tile_mr: MessageReader<PurchaseTileMessage>,
    mut set_tile_mw: MessageWriter<SetTileMessage>,
    mut add_money_mw: MessageWriter<AddMoneyMessage>,
    money: Res<Money>,
) {
    for msg in purchase_tile_mr.read() {
        let cost = msg.tiletype.get_cost();
        if cost > money.0 {
            // Not enough money to purchase the tile
            continue;
        }
        set_tile_mw.write(SetTileMessage {
            pos: msg.pos,
            tiletype: msg.tiletype.clone(),
        });
        add_money_mw.write(AddMoneyMessage {
            increase: -cost
        });
    }
}
