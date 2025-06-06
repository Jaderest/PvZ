use bevy::prelude::*;

use crate::game::*;
use crate::model::components::GameState;

pub fn run() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}