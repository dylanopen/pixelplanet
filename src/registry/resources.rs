use bevy::app::App;

use crate::components::{ResidentialVariant, RoadVariant, TileType, ToolType};
use crate::resources::{self, ActiveToolType, Toolbar};

pub fn register_resources(app: &mut App) {
    app.init_resource::<resources::CurrentHoveredTile>();
    app.init_resource::<resources::Tilemap>();
    app.init_resource::<resources::Money>();
    app.insert_resource(Toolbar {
        slots: vec![
            Some(ToolType::Select),
            Some(ToolType::PlaceTile(TileType::Road(RoadVariant(0)))),
            Some(ToolType::PlaceTile(TileType::Residential(
                ResidentialVariant(0),
            ))),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    });
    app.insert_resource(ActiveToolType(ToolType::PlaceTile(TileType::Road(
        RoadVariant(0),
    ))));
}
