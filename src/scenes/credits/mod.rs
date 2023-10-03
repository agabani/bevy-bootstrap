use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        let state = super::ScenesState::Credits;

        app.add_systems(OnEnter(state), spawn_camera)
            .add_systems(OnExit(state), systems::despawn_recursive::<Camera>);

        #[cfg(feature = "dev")]
        app.register_type::<Camera>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera, Camera2dBundle::default()));
}
