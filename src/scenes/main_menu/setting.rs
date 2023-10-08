use bevy::{ecs::schedule::SystemConfigs, prelude::*};

use crate::systems;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct Screen;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct AccessibilityOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct AudioOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct BackButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct ControlOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct DisplayOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct GameplayOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct GraphicsOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct PCControlOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct ResetToDefaultsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct SelectButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct UserInterfaceOptionsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct Title;

pub(super) fn on_pressed_swap_panels<I: Component, B: systems::Blueprint + 'static>(
) -> SystemConfigs {
    (
        systems::on_pressed_despawn_descendants::<I, Title>,
        systems::on_pressed_spawn_descendant::<I, Title, B>,
    )
        .chain()
}

pub(super) fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            Name::new("Settings"),
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
                background_color: Color::RED.into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Main"),
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        background_color: Color::ORANGE_RED.into(),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("Sidebar"),
                            NodeBundle {
                                style: Style {
                                    height: Val::Percent(100.0),
                                    width: Val::Percent(100.0),
                                    max_width: Val::Px(280.0),
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                background_color: Color::YELLOW.into(),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            use systems::Blueprint;
                            DisplayOptionsButtonBlueprint::template(parent, asset_server);
                            GraphicsOptionsButtonBlueprint::template(parent, asset_server);
                            AudioOptionsButtonBlueprint::template(parent, asset_server);
                            GameplayOptionsButtonBlueprint::template(parent, asset_server);
                            ControlOptionsButtonBlueprint::template(parent, asset_server);
                            PCControlOptionsButtonBlueprint::template(parent, asset_server);
                            AccessibilityOptionsButtonBlueprint::template(parent, asset_server);
                            UserInterfaceOptionsButtonBlueprint::template(parent, asset_server);
                        });

                    parent
                        .spawn((
                            Name::new("Content"),
                            NodeBundle {
                                style: Style {
                                    height: Val::Percent(100.0),
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                background_color: Color::YELLOW_GREEN.into(),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Name::new("Title"),
                                    Title,
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(100.0),
                                            width: Val::Percent(100.0),
                                            max_height: Val::Px(240.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::End,
                                            ..Default::default()
                                        },
                                        background_color: Color::GREEN.into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    use systems::Blueprint;
                                    DisplayOptionsTitleBlueprint::template(parent, asset_server);
                                });

                            parent.spawn((
                                Name::new("Content"),
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
                            ));
                        });
                });

            parent
                .spawn((
                    Name::new("Footer"),
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            max_height: Val::Px(48.0),
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
                    template_footer_button(
                        parent,
                        asset_server,
                        ResetToDefaultsButton,
                        "Reset to Defaults",
                        "RESET TO DEFAULTS",
                    );
                    template_footer_button(parent, asset_server, SelectButton, "Select", "SELECT");
                    template_footer_button(parent, asset_server, BackButton, "Back", "BACK");
                });
        });
}

fn template_footer_button<T: Component>(
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

pub(super) struct DisplayOptionsButtonBlueprint;

impl systems::Blueprint for DisplayOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            DisplayOptionsButton,
            "Display Options",
            "DISPLAY OPTIONS",
        );
    }
}

pub(super) struct GraphicsOptionsButtonBlueprint;

impl systems::Blueprint for GraphicsOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            GraphicsOptionsButton,
            "Graphics Options",
            "GRAPHICS OPTIONS",
        );
    }
}

pub(super) struct AudioOptionsButtonBlueprint;

impl systems::Blueprint for AudioOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            AudioOptionsButton,
            "Audio Options",
            "AUDIO OPTIONS",
        );
    }
}

pub(super) struct GameplayOptionsButtonBlueprint;

impl systems::Blueprint for GameplayOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            GameplayOptionsButton,
            "Gameplay Options",
            "GAMEPLAY OPTIONS",
        );
    }
}

pub(super) struct ControlOptionsButtonBlueprint;

impl systems::Blueprint for ControlOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            ControlOptionsButton,
            "Control Options",
            "CONTROL OPTIONS",
        );
    }
}

pub(super) struct PCControlOptionsButtonBlueprint;

impl systems::Blueprint for PCControlOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            PCControlOptionsButton,
            "PC Control Options",
            "PC CONTROL OPTIONS",
        );
    }
}

pub(super) struct AccessibilityOptionsButtonBlueprint;

impl systems::Blueprint for AccessibilityOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            AccessibilityOptionsButton,
            "Accessibility Options",
            "ACCESSIBILITY OPTIONS",
        );
    }
}

pub(super) struct UserInterfaceOptionsButtonBlueprint;

impl systems::Blueprint for UserInterfaceOptionsButtonBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        sidebar_button_blueprint_template(
            parent,
            asset_server,
            UserInterfaceOptionsButton,
            "User Interface Options",
            "USER INTERFACE OPTIONS",
        );
    }
}

fn sidebar_button_blueprint_template<T: Component>(
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
                    justify_content: JustifyContent::End,
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
                            font_size: 24.0,
                        },
                        value: text.into(),
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub(super) struct DisplayOptionsTitleBlueprint;

impl systems::Blueprint for DisplayOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "DISPLAY OPTIONS");
    }
}

pub(super) struct GraphicsOptionsTitleBlueprint;

impl systems::Blueprint for GraphicsOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "GRAPHICS OPTIONS");
    }
}

pub(super) struct AudioOptionsTitleBlueprint;

impl systems::Blueprint for AudioOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "AUDIO OPTIONS");
    }
}

pub(super) struct GameplayOptionsTitleBlueprint;

impl systems::Blueprint for GameplayOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "GAMEPLAY OPTIONS");
    }
}

pub(super) struct ControlOptionsTitleBlueprint;

impl systems::Blueprint for ControlOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "CONTROL OPTIONS");
    }
}

pub(super) struct PCControlOptionsTitleBlueprint;

impl systems::Blueprint for PCControlOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "PC CONTROL OPTIONS");
    }
}

pub(super) struct AccessibilityOptionsTitleBlueprint;

impl systems::Blueprint for AccessibilityOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "ACCESSIBILITY OPTIONS");
    }
}

pub(super) struct UserInterfaceOptionsTitleBlueprint;

impl systems::Blueprint for UserInterfaceOptionsTitleBlueprint {
    fn template(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        title_blueprint_template(parent, asset_server, "USER INTERFACE OPTIONS");
    }
}

fn title_blueprint_template(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: impl Into<String>,
) {
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
}
