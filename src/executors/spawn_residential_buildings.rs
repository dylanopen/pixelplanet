use bevy::ecs::{message::MessageReader, system::Commands};

use crate::{components::ResidentialBuilding, messages::CreateResidentialBuildingMessage};

pub fn spawn_residential_buildings(
    mut create_residential_building_mr: MessageReader<CreateResidentialBuildingMessage>,
    mut commands: Commands,
) {
    for msg in create_residential_building_mr.read() {
        let pos = msg.pos;
        let residents = msg.residents;
        let capacity = msg.capacity;

        commands.spawn(ResidentialBuilding {
            pos,
            residents,
            capacity,
        });
    }
}
