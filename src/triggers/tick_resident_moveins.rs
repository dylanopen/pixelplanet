use bevy::ecs::system::{Query, Res};

use crate::{components::ResidentialBuilding, resources::MoveinRate};

pub fn tick_resident_moveins(
    mut resident_buildings: Query<&mut ResidentialBuilding>,
    movein_rate: Res<MoveinRate>,
) {
    let mut residents = movein_rate.rate as u32;
    for mut building in resident_buildings.iter_mut() {
        let capacity = building.capacity - building.residents;
        let to_add = capacity.min(residents);
        building.residents += to_add;
        residents -= to_add;
        if residents == 0 {
            break;
        }
    }
}
