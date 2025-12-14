use bevy::app::{App, Plugin, Startup, Update};

use crate::message_readers::{set_tile::set_tile_in_tilemap, update_tile::update_tiles, update_tile_model::update_tile_models};

pub struct TileRenderPlugin;

impl Plugin for TileRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tiles);
        app.add_systems(Update, set_tile_in_tilemap);
        app.add_systems(Update, update_tile_models);
    }
}
