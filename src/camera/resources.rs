use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraCanvas {
    pub margin: UiRect,
    pub scale: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for CameraCanvas {
    fn default() -> Self {
        Self {
            margin: UiRect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(0.),
                bottom: Val::Px(0.),
            },
            scale: 1.0,
            width: 0.0,
            height: 0.0,
        }
    }
}
