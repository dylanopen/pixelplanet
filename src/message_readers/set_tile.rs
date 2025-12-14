use bevy::ecs::{message::{MessageReader, MessageWriter}, system::ResMut};

use crate::{messages::{SetTileMessage, UpdateTileMessage, UpdateTileModelMessage}, resources::Tilemap};

pub fn set_tile_in_tilemap(
    mut set_tile_mr: MessageReader<SetTileMessage>,
    mut tilemap: ResMut<Tilemap>,
    mut update_tile_mw: MessageWriter<UpdateTileMessage>,
    mut update_tile_model_mw: MessageWriter<UpdateTileModelMessage>,
) {
    for msg in set_tile_mr.read() {
        let pos = msg.pos;
        let tile = msg.tile.clone();
        tilemap.set_tile(pos, Some(tile), &mut update_tile_mw, &mut update_tile_model_mw);
    }
}
