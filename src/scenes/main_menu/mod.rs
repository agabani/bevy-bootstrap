use bevy::{app::AppExit, prelude::*};

use crate::prelude::*;

use super::ScenesState;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        let state = super::ScenesState::MainMenu;

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
                    on_pressed_go_to_state::<CreditsButton>(ScenesState::MainMenu),
                    on_pressed_go_to_state::<NewGameButton>(ScenesState::MainMenu),
                    on_pressed_go_to_state::<SettingsButton>(ScenesState::MainMenu),
                    on_pressed_quit::<ExitGameButton>,
                )
                    .run_if(in_state(state)),
            );

        #[cfg(feature = "dev")]
        app.register_type::<Camera>()
            .register_type::<CreditsButton>()
            .register_type::<ExitGameButton>()
            .register_type::<NewGameButton>()
            .register_type::<SettingsButton>()
            .register_type::<UserInterface>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct CreditsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct ExitGameButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct NewGameButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct SettingsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct UserInterface;

fn on_pressed_go_to_state<T: Component>(
    state: ScenesState,
) -> impl Fn(ResMut<NextState<ScenesState>>, Query<&Interaction, (Changed<Interaction>, With<T>)>) {
    move |mut next_state, query| {
        for &interaction in &query {
            if interaction == Interaction::Pressed {
                next_state.set(state);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn on_pressed_quit<T: Component>(
    mut events: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            events.send(AppExit);
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
                                &asset_server,
                                NewGameButton,
                                "New Game",
                                "NEW GAME",
                            );

                            template_menu_button(
                                parent,
                                &asset_server,
                                SettingsButton,
                                "Settings",
                                "SETTINGS",
                            );

                            template_menu_button(
                                parent,
                                &asset_server,
                                CreditsButton,
                                "Credits",
                                "CREDITS",
                            );

                            #[cfg(not(target_family = "wasm"))]
                            template_menu_button(
                                parent,
                                &asset_server,
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
