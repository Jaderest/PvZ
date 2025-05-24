use bevy::prelude::*;

use crate::model::tile::Lawn;
use crate::systems::camera::setup_camera;
use crate::systems::lawn::setup_lawn;
use crate::config::*;
use crate::systems::mouse_control::handle_clicks;
use crate::systems::plant_manage::PlantPlugin;
use crate::systems::sun_manage::SunPlugin;
use crate::systems::zombie_manage::ZombiePlugin;
use crate::view::pvz_ui::*;
use crate::systems::projectile_manage::ProjectilePlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Lawn::default())
        .add_plugins(ConfigPlugin)
        .add_plugins(PlantPlugin)
        .add_plugins(SunPlugin)
        .add_plugins(MyUIPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(ZombiePlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lawn)
        .add_systems(Update, handle_clicks)
        .run();
}