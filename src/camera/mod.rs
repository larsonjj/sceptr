use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraPlugin};

mod systems;

use systems::*;

pub struct CameraPlugin;

// This plugin is responsible to control the game audio
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_system(setup_camera.on_startup());
    }
}
