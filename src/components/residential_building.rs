use bevy::{ecs::component::Component, math::IVec2};

#[derive(Debug, Clone, Component)]
pub struct ResidentialBuilding {
    pub pos: IVec2,
    pub residents: u32,
    pub capacity: u32,
}
