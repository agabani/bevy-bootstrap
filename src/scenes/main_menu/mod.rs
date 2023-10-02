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
        });
}
