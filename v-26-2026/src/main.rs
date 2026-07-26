use bevy::{DefaultPlugins, app::App};
use custom_plugins_bevy::hello_plugin::HelloPlugin;

/// Initiation of the entry point for v-26-2026
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //this is a custom plugin created as a seperate rust library.
        .add_plugins(HelloPlugin)
        .run();
}
