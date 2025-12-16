mod messages;

use bevy::app::App;

pub fn register(mut app: App) -> App {
    messages::register_messages(&mut app);
    app
}

