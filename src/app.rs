use bevy::prelude::*;
use bevy::scene::ron::de;

use crate::config::*;
use crate::model::plant::*;
use crate::model::plant_events::*;
use crate::model::projectile_events::*;
use crate::model::sun::*;
use crate::model::sun_events::*;
use crate::model::zombie::*;
use crate::model::zombie_events::*;
use crate::model::events::*;

use crate::view::play_animation::*;
use crate::view::pvz_ui::*;

use crate::systems::collision_check::*;
use crate::systems::keyboard_control::*;
use crate::systems::plant_manage::*;
use crate::systems::projectile_manage::*;
use crate::systems::sun_manage::*;
use crate::systems::zombie_manage::*;

use crate::model::events::PeaHitZombieEvent;
use crate::model::tile::Lawn;
use crate::systems::camera::setup_camera;
use crate::systems::lawn::setup_lawn;
use crate::systems::mouse_control::handle_clicks;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Configurations
        .insert_resource(Lawn::default())
        .insert_resource(GameConfig::default())
        .insert_resource(GameType::default())
        .insert_resource(ControlState::default())
        .insert_resource(PlantType::default())
        .insert_resource(WindowResolution::default())
        .add_systems(Startup, setup_window_size)
        // basic systems
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lawn)
        .add_systems(Update, handle_clicks)
        // UI management
        .add_systems(Startup, setup_bank_ui)
        .add_systems(Update, update_sun_bank_ui)
        .add_systems(Update, card_click_system)
        .add_systems(Update, card_plant_event)
        .add_systems(Update, shovel_click_system)
        .add_systems(Update, shovel_plant_event)
        // Plant management
        .insert_resource(PlantCost::default())
        .add_event::<SpawnPlantEvent>()
        .add_event::<ShovelPlantEvent>()
        .add_event::<SuccessSpawnPlantEvent>()
        .add_event::<FailedSpawnPlantEvent>()
        .add_event::<SpawnFlowerSunEvent>()
        .add_systems(Update, (
            spawn_plant,
            shovel_plant,
            sunflower_produce,
            peashooter_shoot,
            play_plant_animation,
            despawn_plant,
        ))
        // Sun management
        .insert_resource(GlobalSunTimer::default())
        .insert_resource(SunAmount::default())
        .add_event::<PickupSunEvent>()
        .add_event::<SpawnFlowerSunEvent>()
        .add_event::<SunChangeEvent>()
        .add_systems(Update, sun_produce_sun)
        .add_systems(Update, sun_add)
        .add_systems(Update, sun_consume)
        .add_systems(Update, sun_despawn_with_time)
        .add_systems(Update, flower_produce_sun)
        .add_systems(Update, sun_fall_system)
        .add_systems(Update, flower_sun_fall_system)
        // Projectile Zombie management
        .add_event::<PeaSpawnEvent>()
        .add_event::<ZombieSpawnEvent>()
        .add_event::<PeaHitZombieEvent>()
        .add_systems(Update, keyboard_spawn_zombie)
        .add_systems(
            Update,
            (
                spawn_pea,
                move_pea,
                spawn_zombie,
                zombie_move,
                detect_pea_zombie_collision,
                handle_pea_hit_zombie,
                time_despawn_pea,
                despawn_zombie
            ),
        )
        // Zombie Plant 
        .add_event::<ZombieCollidePlantEvent>()
        .add_event::<PlantReceiveDamageEvent>()
        .add_event::<ZombieTargetNotExistEvent>()
        .add_systems(Update, (
            detect_zombie_plant_collision,
            handle_zombie_collide_plant,
            zombie_attack_plant,
            zombie_recover_walk_system,
            plant_receive_damage,
        ))
        .add_systems(Update, play_zombie_animation)
        .run();
}
