mod temp;

use bevy::{DefaultPlugins, app::{App, Startup}};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, temp::spawn_example_cube);
    app.run();
}
