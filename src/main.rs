mod temp;

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_map_camera::MapCameraPlugin);
    app.add_plugins(pixelplanet::camera::CameraPlugin);
    app.add_plugins(pixelplanet::light::LightPlugin);
    app.add_plugins(pixelplanet::terrain::TerrainPlugin);
    app.add_plugins(pixelplanet::selector::SelectorPlugin);
    app.add_systems(Startup, temp::spawn_example_cube);
    app.run();
}
