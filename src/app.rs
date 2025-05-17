use bevy::prelude::*;

use crate::model::tile::Lawn;
use crate::systems::camera::setup_camera;
use crate::systems::lawn::setup_lawn;
use crate::config::*;
use crate::systems::mouse_control::handle_clicks;
use crate::systems::plant_manage::PlantPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Lawn::default())
        .add_plugins(ConfigPlugin)
        .add_plugins(PlantPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lawn)
        .add_systems(Update, handle_clicks)
        .run();
}