use bevy::math::{IVec2, Vec3};

pub const TILEMAP_SIZE: IVec2 = IVec2::new(64, 64);
pub const MODEL_SIZE: Vec3 = Vec3::splat(64.0);

pub const IVEC2_UP: IVec2 = IVec2::new(0, 1);
pub const IVEC2_DOWN: IVec2 = IVec2::new(0, -1);
pub const IVEC2_LEFT: IVec2 = IVec2::new(-1, 0);
