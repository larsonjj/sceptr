use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

use crate::{REFERENCE_RESOLUTION_HEIGHT, REFERENCE_RESOLUTION_WIDTH};

pub fn setup_camera(mut commands: Commands) {
    let mut camera = PixelCameraBundle::from_resolution(
        REFERENCE_RESOLUTION_WIDTH as i32,
        REFERENCE_RESOLUTION_HEIGHT as i32,
    );
    camera.transform = Transform::from_xyz(
        REFERENCE_RESOLUTION_WIDTH / 2.0,
        REFERENCE_RESOLUTION_HEIGHT / 2.0,
        1000.0,
    );

    commands.spawn(camera);
}
