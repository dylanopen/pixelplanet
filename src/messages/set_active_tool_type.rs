use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone)]
pub struct SetActiveToolTypeMessage {
    pub slot_index: usize,
}

