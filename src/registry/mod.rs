mod messages;
mod resources;
mod startups;
mod triggers;
mod forwarders;
mod executors;

use bevy::app::App;

pub fn register(mut app: App) -> App {
    messages::register_messages(&mut app);
    resources::register_resources(&mut app);
    startups::register_startups(&mut app);
    triggers::register_triggers(&mut app);
    forwarders::register_forwarders(&mut app);
    executors::register_executors(&mut app);

    app
}

