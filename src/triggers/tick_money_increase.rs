use crate::components::ResidentialBuilding;
use crate::messages::AddMoneyMessage;
use crate::resources::ResidentialTax;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::{Query, Res};
use bevy::time::Time;

pub fn tick_money_increase(
    time: Res<Time>,
    mut set_money_mw: MessageWriter<AddMoneyMessage>,
    residential_buildings: Query<&ResidentialBuilding>,
    residential_tax: Res<ResidentialTax>,
) {
    for building in residential_buildings.iter() {
        let tax = residential_tax.rate;
        let residents = building.residents as f64;
        let increase = time.delta_secs_f64() * residents * tax;
        set_money_mw.write(AddMoneyMessage { increase });
    }
}
