use bevy::{ecs::message::Message, math::IVec2};

#[derive(Message)]
pub struct ClickTileMessage {
    pub pos: IVec2,
}

impl ClickTileMessage {
    pub fn new(pos: IVec2) -> Self {
        ClickTileMessage { pos }
    }
}

impl From<IVec2> for ClickTileMessage {
    fn from(pos: IVec2) -> Self {
        ClickTileMessage { pos }
    }
}

impl From<(i32, i32)> for ClickTileMessage {
    fn from(pos: (i32, i32)) -> Self {
        ClickTileMessage {
            pos: IVec2::new(pos.0, pos.1),
        }
    }
}
