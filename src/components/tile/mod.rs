pub mod road_variant;

pub use road_variant::RoadVariant;

use bevy::{ecs::{component::Component, resource::Resource}, math::IVec2};

use crate::consts::TILEMAP_SIZE;

pub struct Tile {
    pub tiletype: TileType,
}

pub enum TileType {
    None,
    Road(RoadVariant),
}

#[derive(Resource)]
pub struct Tilemap {
    pub size: IVec2,
    pub tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn new(size: IVec2) -> Self {
        let mut tiles = Vec::new();
        for y in 0..size.y {
            for x in 0..size.x {
                tiles.push(Tile {
                    tiletype: TileType::None,
                });
            }
        }
        Tilemap { size, tiles }
    }

    pub fn get_tile(&self, pos: IVec2) -> Option<&Tile> {
        let index = pos.y * self.size.x + pos.x;
        self.tiles.get(index as usize)
    }

    pub fn get_tile_mut(&mut self, pos: IVec2) -> Option<&mut Tile> {
        let index = pos.y * self.size.x + pos.x;
        self.tiles.get_mut(index as usize)
    }

    pub fn is_empty(&self, pos: IVec2) -> Option<bool> {
        let index = pos.y * self.size.x + pos.x;
        let tile = self.tiles.get(index as usize);
        if let Some(t) = tile {
            match t.tiletype {
                TileType::None => Some(true),
                _ => Some(false),
            }
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, pos: IVec2, tile: Tile) {
        let index = pos.y * self.size.x + pos.x;
        if let Some(t) = self.tiles.get_mut(index as usize) {
            *t = tile;
        }
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Tilemap::new(TILEMAP_SIZE)
    }
}

