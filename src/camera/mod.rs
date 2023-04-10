use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraPlugin};

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct CameraPlugin;

// This plugin is responsible to control the game audio
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraCanvas>()
            .add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_system(setup_camera.on_startup())
            .add_system(resize_camera_canvas.in_base_set(CoreSet::PostUpdate));
    }
}
