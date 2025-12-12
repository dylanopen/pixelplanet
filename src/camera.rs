use bevy::{
    app::{Plugin, Startup},
    camera::Camera3d,
    ecs::component::Component,
    math::Vec3,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

fn spawn_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

