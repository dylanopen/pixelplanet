use crate::messages::AddMoneyMessage;
use crate::resources::Money;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Res;
use bevy::time::Time;

pub fn tick_money_increase(
    money: Res<Money>,
    time: Res<Time>,
    mut set_money_mw: MessageWriter<AddMoneyMessage>,
) {
    let increase = time.delta_secs_f64() * 5.0; // increase money by 5 per second
    set_money_mw.write(AddMoneyMessage { increase });
}
