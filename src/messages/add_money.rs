use bevy::ecs::message::Message;

#[derive(Message)]
pub struct AddMoneyMessage {
    pub increase: f64,
}
