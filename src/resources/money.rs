use std::fmt::Display;

use bevy::ecs::resource::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct Money(pub f32);

impl Money {
    pub fn new(amount: f32) -> Self {
        Money(amount)
    }

    pub fn add(&mut self, amount: f32) {
        self.0 += amount;
    }

    pub fn subtract(&mut self, amount: f32) {
        self.0 -= amount;
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

impl Default for Money {
    fn default() -> Self {
        Money(1000.0)
    }
}
