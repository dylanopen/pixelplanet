use bevy::ecs::message::Message;

#[derive(Message)]
pub struct SetMoneyMessage {
    pub new: f64,
}
