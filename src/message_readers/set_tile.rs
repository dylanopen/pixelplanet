use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::ResMut,
};

use crate::{
    messages::{SetTileMessage, UpdateTileMessage, UpdateTileModelMessage},
    resources::Tilemap,
};

pub fn set_tile_in_tilemap(
    mut set_tile_mr: MessageReader<SetTileMessage>,
    mut tilemap: ResMut<Tilemap>,
    mut update_tile_mw: MessageWriter<UpdateTileMessage>,
) {
    for msg in set_tile_mr.read() {
        let pos = msg.pos;
        let tiletype = msg.tiletype.clone();
        tilemap.set_tile_type(
            pos,
            tiletype,
            &mut update_tile_mw,
        );
    }
}
