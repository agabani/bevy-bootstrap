#![warn(clippy::pedantic)]

pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod prelude;
pub(crate) mod scenes;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((bevy_config::BevyConfigDefaultPlugin, scenes::ScenesPlugin));

        #[cfg(feature = "dev")]
        app.add_plugins(dev::DevPlugin);
    }
}
