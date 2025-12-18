use bevy::{
    asset::AssetServer,
    ecs::{
        message::MessageReader,
        system::{Commands, ResMut},
    },
    math::Vec3, scene::SceneRoot, transform::components::Transform
};

use crate::{consts::MODEL_SIZE, messages::UpdateTileModelMessage, resources::Tilemap};

pub fn update_tile_models(
    mut commands: Commands,
    mut update_tile_model_mr: MessageReader<UpdateTileModelMessage>,
    asset_server: ResMut<AssetServer>,
    mut tilemap: ResMut<Tilemap>,
) {
    for msg in update_tile_model_mr.read() {
        let pos = msg.pos;
        let tile = tilemap.get_tile_mut(pos).unwrap();
        let tile_model = tile.get_model(&asset_server).unwrap();
        if let Some(entity) = tile.entity {
            commands.entity(entity).despawn();
        }
        let new_entity = commands
            .spawn((
                SceneRoot(tile_model),
                Transform::from_translation(Vec3::new(pos.x as f32 + 0.5, 1.0, pos.y as f32 + 0.5))
                    .with_scale(Vec3::splat(1.0) / MODEL_SIZE),
            ))
            .id();
        tile.entity = Some(new_entity);
    }
}
