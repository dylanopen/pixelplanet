use bevy::ecs::message::{MessageReader, MessageWriter};

use crate::{components::TileType, messages::{CreateResidentialBuildingMessage, SetTileMessage}};

pub fn spawn_building_entities(
    mut set_tile_mr: MessageReader<SetTileMessage>,
    mut create_residential_building_mw: MessageWriter<CreateResidentialBuildingMessage>,
) {
    for msg in set_tile_mr.read() {
        let pos = msg.pos;
        match &msg.tiletype {
            TileType::Residential(variant) => {
                let residents = 0;
                let capacity = variant.get_capacity();
                create_residential_building_mw.write(CreateResidentialBuildingMessage {
                    pos, residents, capacity
                });
            },
            _ => {} // no need to raise events on other tile types
        }
    }
}
