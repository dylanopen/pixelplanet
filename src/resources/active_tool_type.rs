use bevy::ecs::resource::Resource;

use crate::components::ToolType;

#[derive(Resource)]
pub struct ActiveToolType(pub ToolType);
