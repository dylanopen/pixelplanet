#[derive(Debug, Clone)]
pub enum RoadVariant {
    StraightNS,
    StraightEW,
}

impl RoadVariant {
    pub fn is_straight(&self) -> bool {
        matches!(self, RoadVariant::StraightNS | RoadVariant::StraightEW)
    }

    pub fn get_model_name(&self) -> String {
        match self {
            RoadVariant::StraightNS => "road1".to_owned(),
            RoadVariant::StraightEW => "road2".to_owned(),
        }
    }
}

