pub(crate) mod main_menu;
pub(crate) mod title;

use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ScenesState>()
            .add_plugins((main_menu::MainMenuPlugin, title::TitlePlugin));
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, States)]
pub(crate) enum ScenesState {
    MainMenu,
    #[default]
    Title,
}
