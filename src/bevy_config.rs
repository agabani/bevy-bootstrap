use bevy::{prelude::*, window::PresentMode};

#[allow(clippy::module_name_repetitions)]
pub(crate) struct BevyConfigDefaultPlugin;

impl Plugin for BevyConfigDefaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                resolution: (1600.0, 900.0).into(),
                title: "Bevy Bootstrap".into(),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

#[allow(clippy::module_name_repetitions)]
pub(crate) struct BevyConfigMinimalPlugin;

impl Plugin for BevyConfigMinimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
    }
}
