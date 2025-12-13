mod temp;

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
};
use bevy_map_camera::MapCameraPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(MapCameraPlugin);
    app.add_plugins(pixelplanet::camera::CameraPlugin);
    app.add_plugins(pixelplanet::light::LightPlugin);
    app.add_systems(Startup, temp::spawn_example_cube);
    app.run();
}
