use bevy::{DefaultPlugins, app::App};

use pixelplanet::registry::register;
use pixelplanet::{messages, plugins::*, resources};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_map_camera::MapCameraPlugin);
    app.add_plugins(bevy_vox_scene::VoxScenePlugin::default());

    let mut app = register(app);

    app.add_plugins(CameraPlugin);
    app.add_plugins(LightPlugin);
    app.add_plugins(TerrainPlugin);
    app.add_plugins(SelectorPlugin);
    app.add_plugins(TileRenderPlugin);
    app.add_plugins(MoneyPlugin);

    app.run();
}
