mod messages;
mod resources;

use bevy::app::App;

pub fn register(mut app: App) -> App {
    messages::register_messages(&mut app);
    resources::register_resources(&mut app);
    app
}

