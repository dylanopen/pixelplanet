use bevy::{ecs::resource::Resource, math::IVec2};

#[derive(Resource, Default)]
pub struct CurrentHoveredTile {
    pub pos: Option<IVec2>,
}
