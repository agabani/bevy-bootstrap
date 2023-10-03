use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        let state = super::ScenesState::MainMenu;

        app.add_systems(OnEnter(state), (spawn_camera, spawn_user_interface))
            .add_systems(
                OnExit(state),
                (
                    despawn_recursive::<Camera>,
                    despawn_recursive::<UserInterface>,
                ),
            );

        #[cfg(feature = "dev")]
        app.register_type::<Camera>()
            .register_type::<UserInterface>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct UserInterface;

#[allow(clippy::needless_pass_by_value)]
fn despawn_recursive<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera, Camera2dBundle::default()));
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_user_interface(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("User Interface"),
            UserInterface,
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                background_color: Color::RED.into(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Left"),
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::End,
                            ..Default::default()
                        },
                        background_color: Color::ORANGE.into(),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("Menu"),
                            NodeBundle {
                                style: Style {
                                    height: Val::Percent(100.0),
                                    width: Val::Percent(100.0),
                                    max_width: Val::Px(400.0),
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                background_color: Color::YELLOW.into(),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Name::new("Logo"),
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(100.0),
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::End,
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        background_color: Color::GREEN.into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new("Text"),
                                        TextBundle {
                                            text: Text {
                                                sections: vec![TextSection {
                                                    style: TextStyle {
                                                        font: asset_server.load(
                                                            "fonts/Noto_Sans/NotoSans-Bold.ttf",
                                                        ),
                                                        font_size: 48.0,
                                                        color: Color::BLACK,
                                                    },
                                                    value: "Bevy Bootstrap".into(),
                                                }],
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                    ));
                                });

                            parent
                                .spawn((
                                    Name::new("Buttons"),
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(100.0),
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            ..Default::default()
                                        },
                                        background_color: Color::BLUE.into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            Name::new("New Game"),
                                            ButtonBundle {
                                                ..Default::default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    alignment: TextAlignment::Left,
                                                    sections: vec![TextSection {
                                                        style: TextStyle {
                                                            color: Color::BLACK,
                                                            font: asset_server.load(
                                                                "fonts/Noto_Sans/NotoSans-Regular.ttf",
                                                            ),
                                                            font_size: 36.0,
                                                        },
                                                        value: "New Game".into(),
                                                    }],
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });
                                        });

                                    parent
                                        .spawn((
                                            Name::new("Settings"),
                                            ButtonBundle {
                                                ..Default::default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    alignment: TextAlignment::Left,
                                                    sections: vec![TextSection {
                                                        style: TextStyle {
                                                            color: Color::BLACK,
                                                            font: asset_server.load(
                                                                "fonts/Noto_Sans/NotoSans-Regular.ttf",
                                                            ),
                                                            font_size: 36.0,
                                                        },
                                                        value: "Settings".into(),
                                                    }],
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });
                                        });

                                    parent
                                        .spawn((
                                            Name::new("Credits"),
                                            ButtonBundle {
                                                ..Default::default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    alignment: TextAlignment::Left,
                                                    sections: vec![TextSection {
                                                        style: TextStyle {
                                                            color: Color::BLACK,
                                                            font: asset_server.load(
                                                                "fonts/Noto_Sans/NotoSans-Regular.ttf",
                                                            ),
                                                            font_size: 36.0,
                                                        },
                                                        value: "Credits".into(),
                                                    }],
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });
                                        });

                                    #[cfg(not(target_family = "wasm"))]
                                    parent
                                        .spawn((
                                            Name::new("Quit"),
                                            ButtonBundle {
                                                ..Default::default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    alignment: TextAlignment::Left,
                                                    sections: vec![TextSection {
                                                        style: TextStyle {
                                                            color: Color::BLACK,
                                                            font: asset_server.load(
                                                                "fonts/Noto_Sans/NotoSans-Regular.ttf",
                                                            ),
                                                            font_size: 36.0,
                                                        },
                                                        value: "Quit".into(),
                                                    }],
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });
                                        });
                                });
                        });
                });

            parent.spawn((
                Name::new("Right"),
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: Color::PURPLE.into(),
                    ..Default::default()
                },
            ));
        });
}
