use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
};

use crate::prelude::*;

use super::ScenesState;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        let state = super::ScenesState::Title;
        let next_state = super::ScenesState::MainMenu;

        app.add_systems(OnEnter(state), (spawn_camera, spawn_user_interface))
            .add_systems(
                OnExit(state),
                (
                    systems::despawn_recursive::<Camera>,
                    systems::despawn_recursive::<UserInterface>,
                ),
            )
            .add_systems(
                Update,
                (
                    on_key_released_go_to_state(next_state),
                    on_mouse_released_go_to_state(next_state),
                    on_touch_ended_go_to_state(next_state),
                )
                    .run_if(in_state(state)),
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

fn on_key_released_go_to_state(
    state: ScenesState,
) -> impl Fn(ResMut<NextState<ScenesState>>, EventReader<KeyboardInput>) {
    move |mut next_state, mut events| {
        for event in &mut events {
            if event.state == ButtonState::Released {
                next_state.set(state);
            }
        }
    }
}

fn on_mouse_released_go_to_state(
    state: ScenesState,
) -> impl Fn(ResMut<NextState<ScenesState>>, EventReader<MouseButtonInput>) {
    move |mut next_state, mut events| {
        for event in &mut events {
            if event.state == ButtonState::Released {
                next_state.set(state);
            }
        }
    }
}

fn on_touch_ended_go_to_state(
    state: ScenesState,
) -> impl Fn(ResMut<NextState<ScenesState>>, EventReader<TouchInput>) {
    move |mut next_state, mut events| {
        for event in &mut events {
            if event.phase == TouchPhase::Ended {
                next_state.set(state);
            }
        }
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
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
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
                                        font: asset_server
                                            .load("fonts/Noto_Sans/NotoSans-Bold.ttf"),
                                        font_size: 96.0,
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
                    Name::new("Call to action"),
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::End,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        TextBundle {
                            style: Style {
                                margin: UiRect {
                                    bottom: Val::Vh(5.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server
                                            .load("fonts/Noto_Sans/NotoSans-Regular.ttf"),
                                        font_size: 36.0,
                                        color: Color::BLACK,
                                    },
                                    value: "Press any button to start".into(),
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ));
                });
        });
}
