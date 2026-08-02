use crate::environment_tooling::camera_controls::camera_global_setup;
#[cfg(debug_assertions)]
use crate::utils::support_utils::dev_pointer_grip;
use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
};
use custom_plugins_bevy::hello_plugin::HelloPlugin;
mod environment_tooling;
mod scene;
mod utils;

/// Initiation of the entry point for v-26-2026
fn main() {
    let mut main_app = App::new();
    main_app.add_plugins(DefaultPlugins);
    #[cfg(debug_assertions)]
    main_app.add_systems(Update, dev_pointer_grip);
    // global-camera is degined here.
    main_app.add_systems(Startup, camera_global_setup);
    main_app
        .add_systems(Startup, scene::opening_scene::scene)
        //this is a custom plugin created as a seperate rust library.
        .add_plugins(HelloPlugin)
        .run();
}
