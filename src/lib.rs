#![warn(clippy::pedantic)]

pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod prelude;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_config::BevyConfigDefaultPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(dev::DevPlugin);
    }
}
