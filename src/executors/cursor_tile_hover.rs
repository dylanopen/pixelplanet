use bevy::{
    camera::visibility::Visibility,
    ecs::{
        message::MessageReader,
        query::With,
        system::{Res, ResMut, Single},
    },
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    components::TileSelectedIndicator, messages::CursorTileHoverMessage,
    resources::{CurrentHoveredTile, Tilemap},
};

pub fn update_tile_selected_indicator(
    mut indicator_query: Single<(&mut Transform, &mut Visibility), With<TileSelectedIndicator>>,
    mut hover_mr: MessageReader<CursorTileHoverMessage>,
    tilemap: Res<Tilemap>
) {
    for hover_msg in hover_mr.read() {
        match hover_msg.pos {
            Some(pos) => {
                if tilemap.is_empty(pos) {
                    *indicator_query.1 = Visibility::Hidden
                } else {
                    indicator_query.0.translation =
                        Vec3::new(pos.x as f32 + 0.5, 1.0, pos.y as f32 + 0.5);
                    *indicator_query.1 = Visibility::Visible;
                }
            }
            None => {
                *indicator_query.1 = Visibility::Hidden;
            }
        }
    }
}

pub fn update_current_hovered_tile(
    mut hover_mr: MessageReader<CursorTileHoverMessage>,
    mut current_hovered_tile: ResMut<CurrentHoveredTile>,
) {
    for hover_msg in hover_mr.read() {
        current_hovered_tile.pos = hover_msg.pos;
    }
}
