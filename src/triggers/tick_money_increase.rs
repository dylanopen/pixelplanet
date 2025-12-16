use crate::messages::SetMoneyMessage;
use crate::resources::Money;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Res;
use bevy::time::Time;

pub fn tick_money_increase(
    money: Res<Money>,
    time: Res<Time>,
    mut set_money_mw: MessageWriter<SetMoneyMessage>,
) {
    let money_increase = time.delta_secs_f64() * 5.0; // increase money by 5 per second
    let new = money.0 + money_increase;
    set_money_mw.write(SetMoneyMessage { new });
    dbg!(money);
}
