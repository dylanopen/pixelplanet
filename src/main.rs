use bevy::{DefaultPlugins, app::App};

use pixelplanet::{messages, plugins::*, resources};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_map_camera::MapCameraPlugin);
    app.add_plugins(bevy_vox_scene::VoxScenePlugin::default());

    messages::registry::register_messages(&mut app);
    resources::registry::init_resources(&mut app);

    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(light::LightPlugin);
    app.add_plugins(terrain::TerrainPlugin);
    app.add_plugins(selector::SelectorPlugin);

    app.run();
}
