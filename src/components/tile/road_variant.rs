pub enum RoadVariant {
    StraightNS,
    StraightEW,
}

impl RoadVariant {
    pub fn is_straight(&self) -> bool {
        matches!(self, RoadVariant::StraightNS | RoadVariant::StraightEW)
    }

    pub fn get_model_name(&self) -> &'static str {
        match self {
            RoadVariant::StraightNS => "road1",
            RoadVariant::StraightEW => "road2",
        }
    }
}

