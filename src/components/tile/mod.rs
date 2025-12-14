pub mod road_variant;

pub use road_variant::RoadVariant;

pub struct Tile {
    pub tiletype: TileType,
}

pub enum TileType {
    None,
    Road(RoadVariant),
}

