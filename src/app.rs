use bevy::prelude::*;

use crate::game::*;
use crate::model::components::GameState;
use crate::systems::camera::setup_camera;
use crate::view::menu_ui::menu_plugin;
use crate::view::result_ui::result_plugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .init_state::<GameState>()
        .add_plugins((menu_plugin, result_plugin))
        .add_plugins(GamePlugin)
        .run();
}
