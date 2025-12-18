pub mod residential_variant;
pub mod road_variant;

use bevy::{
    asset::{AssetServer, Handle}, ecs::{entity::Entity, system::ResMut}, math::IVec2, scene::Scene
};
pub use residential_variant::ResidentialVariant;
pub use road_variant::RoadVariant;

use crate::resources::Tilemap;

#[derive(Debug, Clone)]
pub struct Tile {
    pub tiletype: TileType,
    pub entity: Option<Entity>,
}

impl Tile {
    pub fn new(tiletype: TileType) -> Self {
        Self {
            tiletype,
            entity: None,
        }
    }

    pub fn get_model_name(&self) -> Option<String> {
        match &self.tiletype {
            TileType::Road(variant) => Some(variant.get_model_name()),
            TileType::Residential(variant) => Some(variant.get_model_name()),
        }
    }

    pub fn get_model_path(&self) -> Option<String> {
        let model_name = self.get_model_name()?;
        Some(format!("tiles/{}.vox", model_name))
    }

    pub fn get_model(&self, asset_server: &ResMut<AssetServer>) -> Option<Handle<Scene>> {
        let model_path = self.get_model_path()?;
        let model_handle: Handle<Scene> = asset_server.load(model_path);
        Some(model_handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileType {
    Road(RoadVariant),
    Residential(ResidentialVariant),
}

impl TileType {
    pub fn get_cost(&self) -> f64 {
        // in the future, these costs will be more dynamic and may be read from
        // a resource/config file.
        match self {
            TileType::Road(_) => 50.0,
            TileType::Residential(_) => 100.0,
        }
    }

    pub fn can_place_at(&self, pos: IVec2, tilemap: &Tilemap) -> bool {
        if tilemap.get_tile(pos).is_some() {
            return false; // cannot place on top of existing tile
        }
        let neighbors = tilemap.get_straight_neighbor_positions(pos);
        match self {
            TileType::Road(_) => true, // roads can be placed anywhere
            TileType::Residential(_) => {
            for neighbor_pos in neighbors {
                if let Some(neighbor_tile) = tilemap.get_tile(neighbor_pos) {
                    if matches!(neighbor_tile.tiletype, TileType::Road(_)) {
                        return true; // can place residential next to a road
                    }
                }
            }
            false
            }
        }
    }
}
