use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone)]
pub struct ChangeToolbarSlotMessage {
    pub slot_index: usize,
}
