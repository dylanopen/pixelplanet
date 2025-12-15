use bevy::ecs::{
    message::{MessageReader, MessageWriter},
    system::ResMut,
};

use crate::{
    components::{RoadVariant, TileType},
    consts::{IVEC2_DOWN, IVEC2_LEFT, IVEC2_RIGHT, IVEC2_UP},
    messages::{UpdateTileMessage, UpdateTileModelMessage},
    resources::Tilemap,
};

pub fn update_tiles(
    mut tilemap: ResMut<Tilemap>,
    mut update_tile_mr: MessageReader<UpdateTileMessage>,
    mut update_tile_model_mw: MessageWriter<UpdateTileModelMessage>,
) {
    for msg in update_tile_mr.read() {
        let pos = msg.pos;
        let tile_opt = tilemap.get_tile(pos);
        let Some(tile) = tile_opt else {
            continue;
        };
        dbg!(&tile.tiletype);
        match tile.tiletype {
            TileType::Road(_) => {
                update_road_tile(&mut tilemap, pos, &mut update_tile_model_mw);
            }
        }
    }
}

fn update_road_tile(
    tilemap: &mut Tilemap,
    pos: bevy::math::IVec2,
    update_tile_model_mw: &mut MessageWriter<UpdateTileModelMessage>,
) {
    let neighbor_up = tilemap
        .get_tile(pos + IVEC2_UP)
        .is_some_and(|tile| matches!(tile.tiletype, TileType::Road(_)));
    let neighbor_down = tilemap
        .get_tile(pos + IVEC2_DOWN)
        .is_some_and(|tile| matches!(tile.tiletype, TileType::Road(_)));
    let neighbor_left = tilemap
        .get_tile(pos + IVEC2_LEFT)
        .is_some_and(|tile| matches!(tile.tiletype, TileType::Road(_)));
    let neighbor_right = tilemap
        .get_tile(pos + IVEC2_RIGHT)
        .is_some_and(|tile| matches!(tile.tiletype, TileType::Road(_)));

    let variant = if neighbor_up && neighbor_down && neighbor_left && neighbor_right {
        11
    } else if neighbor_up && neighbor_down && neighbor_left {
        7
    } else if neighbor_up && neighbor_down && neighbor_right {
        9
    } else if neighbor_left && neighbor_right && neighbor_up {
        10
    } else if neighbor_left && neighbor_right && neighbor_down {
        8
    } else if neighbor_up && neighbor_left {
        5
    } else if neighbor_up && neighbor_right {
        4
    } else if neighbor_down && neighbor_left {
        6
    } else if neighbor_down && neighbor_right {
        3
    } else if neighbor_left || neighbor_right {
        2
    } else if neighbor_up || neighbor_down {
        1
    } else {
        0
    };

    if let Some(tile) = tilemap.get_tile_mut(pos) {
        tile.tiletype = TileType::Road(RoadVariant(variant));
        dbg!(&tile.tiletype);
        update_tile_model_mw.write(UpdateTileModelMessage { pos });
    }
}
