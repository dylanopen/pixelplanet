use bevy::{DefaultPlugins, app::App};

use pixelplanet::registry::register;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_map_camera::MapCameraPlugin);
    app.add_plugins(bevy_vox_scene::VoxScenePlugin::default());

    let mut app = register(app);

    app.run();
}
