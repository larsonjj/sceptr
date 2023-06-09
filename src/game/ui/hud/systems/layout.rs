use bevy::prelude::*;

use crate::camera::resources::CameraCanvas;
use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;
use crate::loading::resources::FontAssets;
use crate::loading::resources::TextureAssets;

pub fn spawn_hud(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    textures: Res<TextureAssets>,
    camera_canvas: Res<CameraCanvas>,
) {
    build_hud(&mut commands, &textures, &font_assets, &camera_canvas);
}

pub fn build_hud(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    font_assets: &Res<FontAssets>,
    camera_canvas: &Res<CameraCanvas>,
) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: camera_canvas.margin,
                    size: Size::new(Val::Px(camera_canvas.width), Val::Percent(15.0)),
                    ..HUD_STYLE
                },
                background_color: Color::RED.into(),
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(&font_assets))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn(NodeBundle {
                    style: RHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(&font_assets))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyText {},
                    ));
                });
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_hud_margins(
    mut node_style_query: Query<&mut Style, With<HUD>>,
    camera_canvas: Res<CameraCanvas>,
) {
    if camera_canvas.is_changed() {
        for mut node_style in node_style_query.iter_mut() {
            node_style.margin = camera_canvas.margin;
            node_style.size.width = Val::Px(camera_canvas.width);
        }
    }
}
