use bevy::ecs::message::Message;

use crate::components::ToolType;

#[derive(Message, Debug, Clone)]
pub struct SetActiveToolTypeMessage {
    pub tooltype: ToolType,
}

