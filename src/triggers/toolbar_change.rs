use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Res;
use bevy::input::{ButtonInput, keyboard::KeyCode};

use crate::messages::ChangeToolbarSlotMessage;

pub fn change_toolbar_tool(
    keys: Res<ButtonInput<KeyCode>>,
    mut change_toolbar_slot_mw: MessageWriter<ChangeToolbarSlotMessage>,
) {
    for key in keys.get_just_pressed() {
        let slot_index = match key {
            KeyCode::Digit1 => Some(1),
            KeyCode::Digit2 => Some(2),
            KeyCode::Digit3 => Some(3),
            KeyCode::Digit4 => Some(4),
            KeyCode::Digit5 => Some(5),
            KeyCode::Digit6 => Some(6),
            KeyCode::Digit7 => Some(7),
            KeyCode::Digit8 => Some(8),
            KeyCode::Digit9 => Some(9),
            KeyCode::Digit0 => Some(0),
            _ => None,
        };
        if let Some(index) = slot_index {
            change_toolbar_slot_mw.write(ChangeToolbarSlotMessage { slot_index: index });
        }
    }
}
