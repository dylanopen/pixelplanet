use bevy::{
    ecs::{
        message::MessageReader,
        query::With,
        system::{ResMut, Single},
    },
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    components::TileSelectedIndicator, messages::CursorTileHoverMessage,
    resources::CurrentHoveredTile,
};

pub fn update_tile_selected_indicator(
    mut indicator_transform: Single<&mut Transform, With<TileSelectedIndicator>>,
    mut hover_mr: MessageReader<CursorTileHoverMessage>,
) {
    for hover_msg in hover_mr.read() {
        let transform = indicator_transform.as_mut();
        transform.translation = Vec3::new(
            hover_msg.pos.x as f32 + 0.5,
            1.52,
            hover_msg.pos.y as f32 + 0.5,
        );
    }
}

pub fn update_current_hovered_tile(
    mut hover_mr: MessageReader<CursorTileHoverMessage>,
    mut current_hovered_tile: ResMut<CurrentHoveredTile>,
) {
    for hover_msg in hover_mr.read() {
        current_hovered_tile.pos = Some(hover_msg.pos);
    }
}
