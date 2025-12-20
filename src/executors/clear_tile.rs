use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::{Commands, ResMut},
};

use crate::{
    messages::{ClearTileMessage, UpdateTileMessage},
    resources::Tilemap,
};

pub fn set_tile_blank(
    mut commands: Commands,
    mut set_tile_mr: MessageReader<ClearTileMessage>,
    mut tilemap: ResMut<Tilemap>,
    mut update_tile_mw: MessageWriter<UpdateTileMessage>,
) {
    for msg in set_tile_mr.read() {
        let pos = msg.pos;
        if let Some(tile) = tilemap.get_tile(pos)
            && let Some(entity) = tile.entity
        {
            commands.entity(entity).despawn();
        }
        tilemap.set_tile(pos, None, &mut update_tile_mw);
    }
}
