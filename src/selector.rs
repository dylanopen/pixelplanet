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

#[derive(Message)]
pub struct CursorTileHoverMessage {
    pos: IVec2,
}

#[derive(Component)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)]
pub struct TileSelectedIndicator;

fn spawn_selector_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(0.5, -100.0, 0.5);
    commands.spawn((
        TileSelectedIndicator,
        Mesh3d(meshes.add(Cuboid::new(1.02, 1.02, 1.02))),
        MeshMaterial3d(materials.add(Color::linear_rgba(1.0, 1.0, 1.0, 0.4))),
        transform,
        NotShadowCaster,
    ));
}

fn update_tile_selected_indicator(
    mut indicator_transform: Single<&mut Transform, With<TileSelectedIndicator>>,
    mut hover_mr: MessageReader<CursorTileHoverMessage>,
) {
    for hover_msg in hover_mr.read() {
        let transform = indicator_transform.as_mut();
        transform.translation = Vec3::new(
            hover_msg.pos.x as f32 + 0.5,
            1.52,
            hover_msg.pos.y as f32 + 0.5,
        );
    }
}

fn map_cursor_hover(
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    ground: Single<&GlobalTransform, With<Ground>>,
    mut hover_mw: MessageWriter<CursorTileHoverMessage>,
) {
    let (camera, camera_transform) = *camera_query;
    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let ray = match camera.viewport_to_world(camera_transform, cursor_position) {
        Ok(ray) => ray,
        Err(_) => return,
    };
    let distance =
        match ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up())) {
            Some(distance) => distance,
            None => return,
        };

    let point = ray.get_point(distance).floor().xz();

    hover_mw.write(CursorTileHoverMessage {
        pos: IVec2::new(point.x as i32, point.y as i32),
    });
}
