use bevy::{asset::AssetServer, ecs::{message::MessageReader, system::{Commands, ResMut}}, math::Vec3, scene::SceneRoot, transform::components::Transform};

use crate::messages::click_tile::ClickTileMessage;

pub fn place_road(
    mut commands: Commands,
    mut click_tile_mr: MessageReader<ClickTileMessage>,
    asset_server: ResMut<AssetServer>,
) {
    for msg in click_tile_mr.read() {
        let road_handle = asset_server.load("buildings/road1.vox");
        commands.spawn((
                SceneRoot(road_handle),
                Transform::from_xyz(msg.pos.x as f32 + 0.5, 0.0, msg.pos.y as f32 + 0.5)
                    .with_scale(Vec3::splat(1.0 / 64.0)),
        ));
    }
}
