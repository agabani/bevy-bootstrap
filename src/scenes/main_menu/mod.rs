mod main;
mod setting;

use bevy::{ecs::schedule::SystemConfigs, prelude::*};

use crate::prelude::*;

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
                    systems::on_pressed_exit_app::<main::ExitGameButton>,
                    on_pressed_swap_screens::<main::SettingsButton, setting::Screen>(),
                    on_pressed_swap_screens::<setting::BackButton, main::Screen>(),
                )
                    .run_if(in_state(state)),
            );

        #[cfg(feature = "dev")]
        app.register_type::<Camera>()
            .register_type::<Screen>()
            .register_type::<UserInterface>()
            .register_type::<main::CreditsButton>()
            .register_type::<main::ExitGameButton>()
            .register_type::<main::NewGameButton>()
            .register_type::<main::Screen>()
            .register_type::<main::SettingsButton>()
            .register_type::<setting::BackButton>()
            .register_type::<setting::Screen>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Screen;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct UserInterface;

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
            main::template(parent, &asset_server);
            setting::template(parent, &asset_server);
        });
}

fn on_pressed_swap_screens<B: Component, S: Component>() -> SystemConfigs {
    (
        systems::on_pressed_change_visibility::<B, Screen>(Visibility::Hidden),
        systems::on_pressed_change_visibility::<B, S>(Visibility::Inherited),
    )
        .chain()
}
