use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct MoveinRate {
    pub rate: f64,
}

impl Default for MoveinRate {
    fn default() -> Self {
        MoveinRate { rate: 1.0 }
    }
}

