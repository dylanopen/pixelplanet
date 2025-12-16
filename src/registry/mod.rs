mod messages;
mod resources;
mod startups;
mod executors;
mod forwarders;

use bevy::app::App;

pub fn register(mut app: App) -> App {
    messages::register_messages(&mut app);
    resources::register_resources(&mut app);
    startups::register_startups(&mut app);
    executors::register_executors(&mut app);
    forwarders::register_forwarders(&mut app);

    app
}

