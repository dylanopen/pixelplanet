use bevy::{asset::AssetServer, ecs::{entity::ContainsEntity, message::MessageReader, system::{Commands, Res, ResMut}}, math::Vec3, scene::SceneRoot, transform::components::Transform};

use crate::{messages::UpdateTileModelMessage, resources::Tilemap};

pub fn update_tile_models(
    mut commands: Commands,
    mut update_tile_model_mr: MessageReader<UpdateTileModelMessage>,
    asset_server: ResMut<AssetServer>,
    mut tilemap: ResMut<Tilemap>
) {
    for msg in update_tile_model_mr.read() {
        dbg!("Updating tile model at position: {:?}", msg.pos);
        let pos = msg.pos;
        let tile = tilemap.get_tile_mut(pos).unwrap();
        let tile_model = tile.get_model(&asset_server).unwrap();
        if let Some(entity) = tile.entity {
            commands.entity(entity).despawn();
        }
        let new_entity = commands.spawn((
            SceneRoot(tile_model),
            Transform::from_translation(Vec3::new(
                pos.x as f32 + 0.5,
                0.0,
                pos.y as f32 + 0.5,
            )),
        )).id();
        tile.entity = Some(new_entity);
    }
}
