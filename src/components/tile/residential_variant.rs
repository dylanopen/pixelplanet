#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentialVariant(pub i32);

impl ResidentialVariant {
    pub fn get_model_name(&self) -> String {
        format!("house{}", self.0)
    }
}
