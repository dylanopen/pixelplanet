use bevy::{
    camera::Camera,
    ecs::{
        message::MessageWriter,
        query::With,
        system::{Res, Single},
    },
    math::{IVec2, Vec3Swizzles, primitives::InfinitePlane3d},
    transform::components::GlobalTransform,
    window::Window,
};

use crate::{
    components::{Ground, MainCamera},
    messages::CursorTileHoverMessage,
    resources::Tilemap,
};

pub fn map_cursor_hover(
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    ground: Single<&GlobalTransform, With<Ground>>,
    tilemap: Res<Tilemap>,
    mut hover_mw: MessageWriter<CursorTileHoverMessage>,
) {
    let (camera, camera_transform) = *camera_query;
    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let ray = match camera.viewport_to_world(camera_transform, cursor_position) {
        Ok(ray) => ray,
        Err(_) => return,
    };
    let distance =
        match ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up())) {
            Some(distance) => distance,
            None => return,
        };

    let point = ray.get_point(distance).floor().xz();
    let pos = IVec2::new(point.x as i32, point.y as i32);
    if tilemap.within_range(pos) {
        hover_mw.write(CursorTileHoverMessage { pos: Some(pos) });
    } else {
        hover_mw.write(CursorTileHoverMessage { pos: None });
    }
}
