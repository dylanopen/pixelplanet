#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadVariant(pub i32);

impl RoadVariant {
    pub fn is_straight(&self) -> bool {
        matches!(self.0, 1 | 2)
    }

    pub fn get_model_name(&self) -> String {
        format!("road{}", self.0)
    }
}
