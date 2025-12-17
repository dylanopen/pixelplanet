use bevy::ecs::{message::MessageReader, system::ResMut};

use crate::{messages::SetActiveToolTypeMessage, resources::ActiveToolType};

pub fn set_active_tool_type(
    mut set_active_tool_type_mr: MessageReader<SetActiveToolTypeMessage>,
    mut active_tool_type: ResMut<ActiveToolType>,
) {
    for msg in set_active_tool_type_mr.read() {
        active_tool_type.0 = msg.tooltype.clone();
    }
}
