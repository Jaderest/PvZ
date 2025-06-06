use bevy::prelude::*;

use crate::game::*;
use crate::model::components::GameState;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(GamePlugin)
        .run();
}
