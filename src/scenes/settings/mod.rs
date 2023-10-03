use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let state = super::ScenesState::Settings;

        app.add_systems(OnEnter(state), spawn_camera)
            .add_systems(OnExit(state), despawn_recursive::<Camera>);

        #[cfg(feature = "dev")]
        app.register_type::<Camera>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[allow(clippy::needless_pass_by_value)]
fn despawn_recursive<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera, Camera2dBundle::default()));
}
