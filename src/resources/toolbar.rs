use bevy::ecs::resource::Resource;

use crate::components::ToolType;

#[derive(Resource)]
pub struct Toolbar {
    pub slots: Vec<Option<ToolType>>,
}
