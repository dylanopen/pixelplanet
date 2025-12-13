use bevy::{
    asset::AssetServer, ecs::system::{Commands, ResMut}, math::Vec3, scene::SceneRoot, transform::components::Transform
};

pub fn spawn_example_road(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let road_handle = asset_server.load("buildings/road1.vox");

    commands.spawn((
        SceneRoot(road_handle),
        Transform::from_xyz(16.0, 16.0, 0.0).with_scale(Vec3::splat(1.0 / 64.0)),
    ));
}

