use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct ResidentialTax {
    pub rate: f64,
}

impl Default for ResidentialTax {
    fn default() -> Self {
        ResidentialTax { rate: 0.1 }
    }
}
