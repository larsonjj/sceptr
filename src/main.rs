// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use sceptr::states::*;
use sceptr::systems::*;
use sceptr::LibPlugin;
use sceptr::WINDOW_RESOLUTION_HEIGHT;
use sceptr::WINDOW_RESOLUTION_WIDTH;

fn main() {
    App::new()
        // Bevy
        .add_state::<AppState>()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Learn Bevy Tutorial".to_string(),
                        resolution: (WINDOW_RESOLUTION_WIDTH, WINDOW_RESOLUTION_HEIGHT).into(),
                        canvas: Some("#bevy".to_owned()),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // App Systems
        .add_system(set_window_icon.on_startup())
        .add_system(handle_app_exit_event)
        // My Plugins
        .add_plugin(LibPlugin)
        .run();
}
