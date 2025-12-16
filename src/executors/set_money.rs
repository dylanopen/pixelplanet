use bevy::ecs::message::MessageReader;
use bevy::ecs::system::ResMut;

use crate::messages::SetMoneyMessage;
use crate::resources::Money;

pub fn set_money_res(
    mut set_money_mr: MessageReader<SetMoneyMessage>,
    mut money_res: ResMut<Money>,
) {
    for msg in set_money_mr.read() {
        money_res.0 = msg.new;
    }
}
