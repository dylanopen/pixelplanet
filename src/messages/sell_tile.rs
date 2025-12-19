use bevy::math::IVec2;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone)]
pub struct SellTileMessage {
    pub pos: IVec2,
}
