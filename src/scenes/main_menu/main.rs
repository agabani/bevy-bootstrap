use bevy::prelude::*;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct Screen;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct CreditsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct ExitGameButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct NewGameButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct SettingsButton;

pub(super) fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            Name::new("Main"),
            super::Screen,
            Screen,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::ORANGE.into(),
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
                            min_width: Val::Px(320.0),
                            max_width: Val::Px(640.0),
                            width: Val::Percent(33.3),
                            flex_direction: FlexDirection::Column,
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
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(16.0),
                                    ..Default::default()
                                },
                                background_color: Color::YELLOW.into(),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            template_menu_button(
                                parent,
                                asset_server,
                                NewGameButton,
                                "New Game",
                                "NEW GAME",
                            );

                            template_menu_button(
                                parent,
                                asset_server,
                                SettingsButton,
                                "Settings",
                                "SETTINGS",
                            );

                            template_menu_button(
                                parent,
                                asset_server,
                                CreditsButton,
                                "Credits",
                                "CREDITS",
                            );

                            #[cfg(not(target_family = "wasm"))]
                            template_menu_button(
                                parent,
                                asset_server,
                                ExitGameButton,
                                "Exit Game",
                                "EXIT GAME",
                            );
                        });
                });
        });
}

fn template_menu_button<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    component: T,
    name: impl Into<String>,
    text: impl Into<String>,
) {
    parent
        .spawn((
            Name::new(name.into()),
            component,
            ButtonBundle {
                background_color: Color::rgb(0.078, 0.145, 0.173).into(),
                border_color: Color::rgb(0.6, 0.525, 0.298).into(),
                style: Style {
                    border: UiRect {
                        bottom: Val::Px(2.0),
                        left: Val::Px(2.0),
                        right: Val::Px(2.0),
                        top: Val::Px(2.0),
                    },
                    padding: UiRect {
                        bottom: Val::Px(8.0),
                        left: Val::Px(24.0),
                        right: Val::Px(24.0),
                        top: Val::Px(8.0),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    alignment: TextAlignment::Left,
                    sections: vec![TextSection {
                        style: TextStyle {
                            color: Color::WHITE,
                            font: asset_server.load("fonts/Noto_Sans/NotoSans-Regular.ttf"),
                            font_size: 36.0,
                        },
                        value: text.into(),
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
