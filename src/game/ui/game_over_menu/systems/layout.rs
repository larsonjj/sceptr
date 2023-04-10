// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs

use bevy::prelude::*;

use crate::camera::resources::CameraCanvas;
use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;
use crate::loading::resources::FontAssets;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    camera_canvas: Res<CameraCanvas>,
) {
    build_game_over_menu(&mut commands, &font_assets, &camera_canvas);
}

pub fn build_game_over_menu(
    commands: &mut Commands,
    font_assets: &Res<FontAssets>,
    camera_canvas: &Res<CameraCanvas>,
) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: camera_canvas.margin,
                    size: Size::new(Val::Px(camera_canvas.width), Val::Px(camera_canvas.height)),
                    ..GAME_OVER_MENU_STYLE
                },
                z_index: ZIndex::Local(2), // See Ref. 1
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Final Score Text
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Final Score:",
                                    get_final_score_text_style(&font_assets),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Restart",
                                        get_button_text_style(&font_assets),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(&font_assets),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    #[cfg(not(target_family = "wasm"))]
                    {
                        // Quit Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                QuitButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Quit",
                                            get_button_text_style(&font_assets),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn update_game_over_menu_margins(
    mut node_style_query: Query<&mut Style, With<GameOverMenu>>,
    camera_canvas: Res<CameraCanvas>,
) {
    if camera_canvas.is_changed() {
        for mut node_style in node_style_query.iter_mut() {
            node_style.margin = camera_canvas.margin;
            node_style.size.width = Val::Px(camera_canvas.width);
            node_style.size.height = Val::Px(camera_canvas.height);
        }
    }
}
