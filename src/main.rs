#![warn(clippy::pedantic)]

use bevy::prelude::*;
use bevy_bootstrap::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
