use bevy::prelude::*;

use crate::camera::resources::CameraCanvas;
use crate::loading::resources::FontAssets;
use crate::loading::resources::TextureAssets;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    textures: Res<TextureAssets>,
    camera_canvas: Res<CameraCanvas>,
) {
    build_main_menu(&mut commands, &textures, &font_assets, &camera_canvas);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    font_assets: &Res<FontAssets>,
    camera_canvas: &Res<CameraCanvas>,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: camera_canvas.margin,
                    ..MAIN_MENU_STYLE
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Sceptr",
                                get_title_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
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
                // === Quit Button ===
                parent
                    .spawn((
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        QuitButton {},
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
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
        })
        .id();

    main_menu_entity
}

pub fn update_main_menu_margins(
    mut node_style_query: Query<&mut Style, With<MainMenu>>,
    camera_canvas: Res<CameraCanvas>,
) {
    if camera_canvas.is_changed() {
        for mut node_style in node_style_query.iter_mut() {
            node_style.margin = camera_canvas.margin;
            node_style.size.width = Val::Px(camera_canvas.width);
        }
    }
}
