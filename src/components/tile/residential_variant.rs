#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentialVariant(pub i32);

impl ResidentialVariant {
    pub fn get_model_name(&self) -> String {
        format!("house{}", self.0)
    }

    pub fn get_capacity(&self) -> u32 {
        50 // in the future, this will vary based on the residential variant.
    }
}
