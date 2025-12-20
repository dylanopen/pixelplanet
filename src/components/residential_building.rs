use bevy::math::IVec2;

#[derive(Debug, Clone)]
pub struct ResidentialBuilding {
    pub pos: IVec2,
    pub residents: u32,
    pub capacity: u32,
}
