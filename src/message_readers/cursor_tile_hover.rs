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

