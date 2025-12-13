use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    camera::Camera,
    color::Color,
    ecs::{
        component::Component,
        message::{Message, MessageReader, MessageWriter},
        query::With,
        system::{Commands, ResMut, Single},
    },
    light::NotShadowCaster,
    math::{
        IVec2, Vec3, Vec3Swizzles,
        primitives::{Cuboid, InfinitePlane3d},
    },
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::{camera::MainCamera, terrain::Ground};

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CursorTileHoverMessage>();
        app.add_systems(Startup, spawn_selector_mesh);
        app.add_systems(Update, map_cursor_hover);
        app.add_systems(Update, update_tile_selected_indicator);
    }
}

