use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;

use crate::messages::{AddMoneyMessage, SetMoneyMessage};
use crate::resources::Money;

pub fn set_added_money(
    money: Res<Money>,
    mut add_money_mr: MessageReader<AddMoneyMessage>,
    mut set_money_mw: MessageWriter<SetMoneyMessage>,
) {
    let mut increase = 0.0;
    for msg in add_money_mr.read() {
        increase += msg.increase;
    }
    if increase == 0.0 { return; }
    let new_amount = money.0 + increase;
    set_money_mw.write(SetMoneyMessage { new: new_amount });
}
