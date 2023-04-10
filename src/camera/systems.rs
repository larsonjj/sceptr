use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pixel_camera::{PixelCameraBundle, PixelProjection};

use crate::{REFERENCE_RESOLUTION_HEIGHT, REFERENCE_RESOLUTION_WIDTH};

use super::resources::CameraCanvas;

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

pub fn resize_camera_canvas(
    cameras: Query<&PixelProjection, Changed<PixelProjection>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_canvas: ResMut<CameraCanvas>,
) {
    if window_query.get_single().is_ok() {
        let window = window_query.get_single().unwrap();
        if let Some(projection) = cameras.iter().next() {
            let window_width = window.width() as f32;
            let window_height = window.height() as f32;
            let zoom = projection.zoom as f32;

            let desired_width = projection.desired_width.map(|w| w as f32).unwrap_or(0.0);
            let desired_height = projection.desired_height.map(|h| h as f32).unwrap_or(0.0);
            let desired_width_zoomed = desired_width * zoom;
            let desired_height_zoomed = desired_height * zoom;

            let canvas_origin_x = desired_width_zoomed / 2.0;
            let canvas_origin_y = desired_height_zoomed / 2.0;
            let camera_origin_x = window_width / 2.0;
            let camera_origin_y = window_height / 2.0;

            // populate new camera canvas margin
            camera_canvas.margin = UiRect {
                left: Val::Px((camera_origin_x - canvas_origin_x - (zoom / 2.0)).floor()),
                top: Val::Px((camera_origin_y - canvas_origin_y).floor()),
                right: Val::Px(
                    (camera_origin_x - canvas_origin_x + desired_width_zoomed - window_width
                        + (zoom / 2.0))
                        .abs()
                        .floor(),
                ),
                bottom: Val::Px(
                    (camera_origin_y - canvas_origin_y + desired_height_zoomed - window_height)
                        .abs()
                        .floor(),
                ),
            };
            camera_canvas.scale = zoom;
            camera_canvas.width = desired_width_zoomed;
            camera_canvas.height = desired_height_zoomed + (zoom / 4.0).floor();

            println!(
                "Camera Canvas: {:?} | Scale: {} | Width {} | Height {}",
                camera_canvas.margin,
                camera_canvas.scale,
                camera_canvas.width,
                camera_canvas.height
            )
        }
    }
}
