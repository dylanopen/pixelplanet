use bevy::{asset::{AssetServer, Handle}, ecs::{message::MessageWriter, resource::Resource, system::ResMut}, math::IVec2, scene::Scene};

use crate::{components::{Tile, TileType}, consts::TILEMAP_SIZE, messages::{UpdateTileMessage, UpdateTileModelMessage}};


#[derive(Resource)]
pub struct Tilemap {
    pub size: IVec2,
    pub tiles: Vec<Option<Tile>>,
}

impl Tilemap {
    pub fn new(size: IVec2) -> Self {
        let mut tiles = Vec::new();
        for _ in 0..size.y {
            for _ in 0..size.x {
                tiles.push(None);
            }
        }
        Tilemap { size, tiles }
    }

    pub fn get_tile(&self, pos: IVec2) -> Option<&Tile> {
        let index = pos.y * self.size.x + pos.x;
        self.tiles.get(index as usize)?.as_ref()
    }

    pub fn get_tile_mut(&mut self, pos: IVec2) -> Option<&mut Tile> {
        let index = pos.y * self.size.x + pos.x;
        self.tiles.get_mut(index as usize)?.as_mut()
    }

    pub fn is_empty(&self, pos: IVec2) -> Option<bool> {
        let tile = self.get_tile(pos);
        if let Some(t) = tile {
            match t.tiletype {
                TileType::None => Some(true),
                _ => Some(false),
            }
        } else {
            None
        }
    }

    pub fn set_tile(
        &mut self,
        pos: IVec2,
        tile: Option<Tile>,
        update_tile_mw: &mut MessageWriter<UpdateTileMessage>,
        update_tile_model_mw: &mut MessageWriter<UpdateTileModelMessage>,
    ) -> Option<()> {
        let index = pos.y * self.size.x + pos.x;
        if let Some(t) = self.tiles.get_mut(index as usize) {
            *t = tile;
            self.update_neighbors(pos, update_tile_mw); 
            update_tile_model_mw.write(UpdateTileModelMessage { pos });
            return Some(());
        }
        None
    }

    pub fn get_model(&self, asset_server: &ResMut<AssetServer>, pos: IVec2) -> Option<Handle<Scene>> {
        let tile = self.get_tile(pos)?;
        tile.get_model(asset_server)
    }

    pub fn get_model_name(&self, tile: &Tile) -> Option<String> {
        tile.get_model_name()
    }

    pub fn get_model_path(&self, tile: &Tile) -> Option<String> {
        tile.get_model_path()
    }

    fn update_neighbors(&self, pos: IVec2, update_tile_mw: &mut MessageWriter<UpdateTileMessage>) {
        let directions = [
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 1),
            IVec2::new(1, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 1),
        ];

        for dir in directions.iter() {
            let neighbor_pos = pos + *dir;
            if neighbor_pos.x < 0 { continue; }
            if neighbor_pos.y < 0 { continue; }
            if neighbor_pos.x >= self.size.x { continue; }
            if neighbor_pos.y >= self.size.y { continue; }
            update_tile_mw.write(UpdateTileMessage { pos: neighbor_pos });
        }
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Tilemap::new(TILEMAP_SIZE)
    }
}

