use bevy::prelude::*;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct Screen;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct BackButton;

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
            parent.spawn((
                Name::new("Main"),
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..Default::default()
                },
            ));

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
                        background_color: Color::YELLOW.into(),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("Back"),
                            BackButton,
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
                                        bottom: Val::Px(0.0),
                                        left: Val::Px(24.0),
                                        right: Val::Px(24.0),
                                        top: Val::Px(0.0),
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
                                            font: asset_server
                                                .load("fonts/Noto_Sans/NotoSans-Regular.ttf"),
                                            font_size: 36.0,
                                        },
                                        value: "BACK".into(),
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
        });
}
