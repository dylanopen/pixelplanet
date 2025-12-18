use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;

use crate::components::ToolType;
use crate::{
    messages::{ChangeToolbarSlotMessage, SetActiveToolTypeMessage},
    resources::Toolbar,
};

pub fn change_active_tool(
    mut change_toolbar_slot_mr: MessageReader<ChangeToolbarSlotMessage>,
    mut set_active_tool_type_mw: MessageWriter<SetActiveToolTypeMessage>,
    toolbar: Res<Toolbar>,
) {
    for msg in change_toolbar_slot_mr.read() {
        let tooltype = toolbar
            .slots
            .get(msg.slot_index)
            .unwrap_or(&Some(ToolType::Select))
            .clone()
            .unwrap_or(ToolType::Select);
        set_active_tool_type_mw.write(SetActiveToolTypeMessage { tooltype });
    }
}
