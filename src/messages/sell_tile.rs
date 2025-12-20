use bevy::ecs::message::Message;
use bevy::math::IVec2;

#[derive(Message, Debug, Clone)]
pub struct SellTileMessage {
    pub pos: IVec2,
}
