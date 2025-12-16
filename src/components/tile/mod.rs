pub mod road_variant;

use bevy::{
    asset::{AssetServer, Handle},
    ecs::{entity::Entity, system::ResMut},
    scene::Scene,
};
pub use road_variant::RoadVariant;

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
}

impl TileType {
    pub fn get_cost(&self) -> f64 {
        // in the future, these costs will be more dynamic and may be read from
        // a resource/config file.
        match self {
            TileType::Road(_) => 50.0,
        }
    }
}

