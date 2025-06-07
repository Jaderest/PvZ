use bevy::prelude::*;

use crate::config::*;
use crate::model::components::GameState;
use crate::model::events::*;
use crate::model::level::Level;
use crate::model::plant::*;
use crate::model::plant_events::*;
use crate::model::projectile_events::*;
use crate::model::sun::*;
use crate::model::sun_events::*;
use crate::model::zombie_events::*;

use crate::model::zombie_pole_vaulting::*;
use crate::view::back_ground::setup_game_background;
use crate::view::play_animation::*;
use crate::view::pvz_ui::*;

use crate::systems::collision_check::*;
use crate::systems::keyboard_control::*;
use crate::systems::plant_manage::*;
use crate::systems::projectile_manage::*;
use crate::systems::sun_manage::*;
use crate::systems::zombie_manage::*;
use crate::systems::level_manage::*;

use crate::model::events::PeaHitZombieEvent;
use crate::model::tile::Lawn;
use crate::systems::lawn::setup_lawn;
use crate::systems::mouse_control::handle_clicks;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app// Configurations
            .insert_resource(Lawn::default())
            .insert_resource(GameConfig::default())
            .insert_resource(GameType::default())
            .insert_resource(ControlState::default())
            .insert_resource(PlantType::default())
            .insert_resource(WindowResolution::default())
            .add_systems(OnEnter(GameState::Game), setup_window_size)
            // basic systems
            .add_systems(OnEnter(GameState::Game), setup_lawn)
            .add_systems(OnEnter(GameState::Game), setup_game_background)
            .add_systems(Update, handle_clicks.run_if(in_state(GameState::Game)))
            // UI management
            .add_event::<ClearCardEvent>()
            .add_event::<SetCardCDEvent>()
            .add_systems(OnEnter(GameState::Game), setup_bank_ui)
            .add_systems(Update, update_sun_bank_ui.run_if(in_state(GameState::Game)))
            .add_systems(Update, card_click_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, clear_card_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, card_plant_event.run_if(in_state(GameState::Game)))
            .add_systems(Update, shovel_click_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, shovel_plant_event.run_if(in_state(GameState::Game)))
            .add_systems(Update, card_cd_tick_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, card_cd_update_system.run_if(in_state(GameState::Game)))
            // Plant management
            .insert_resource(PlantCost::default())
            .add_event::<SpawnPlantEvent>()
            .add_event::<ShovelPlantEvent>()
            .add_event::<SuccessSpawnPlantEvent>()
            .add_event::<FailedSpawnPlantEvent>()
            .add_event::<SpawnFlowerSunEvent>()
            .add_systems(
                Update,
                (
                    spawn_plant,
                    shovel_plant,
                    sunflower_produce,
                    peashooter_shoot,
                    play_plant_animation,
                    despawn_plant,
                ),
            )
            // Sun management
            .insert_resource(GlobalSunTimer::default())
            .insert_resource(SunAmount::default())
            .add_event::<PickupSunEvent>()
            .add_event::<SpawnFlowerSunEvent>()
            .add_event::<SunChangeEvent>()
            .add_systems(Update, sun_produce_sun.run_if(in_state(GameState::Game)))
            .add_systems(Update, sun_add.run_if(in_state(GameState::Game)))
            .add_systems(Update, sun_consume.run_if(in_state(GameState::Game)))
            .add_systems(Update, sun_despawn_with_time.run_if(in_state(GameState::Game)))
            .add_systems(Update, flower_produce_sun.run_if(in_state(GameState::Game)))
            .add_systems(Update, sun_fall_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, flower_sun_fall_system.run_if(in_state(GameState::Game)))
            // Projectile Zombie management
            .add_event::<PeaSpawnEvent>()
            .add_event::<ZombieSpawnEvent>()
            .add_event::<PeaHitZombieEvent>()
            .add_event::<ZombieDefenderBrokenEvent>()
            .add_systems(Update, keyboard_spawn_zombie.run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                (
                    spawn_pea,
                    move_pea,
                    spawn_zombie,
                    zombie_move,
                    handle_pole_vaulting_zombie_collide_plant,
                    detect_pea_zombie_collision,
                    handle_pea_hit_zombie,
                    break_zombie_defender,
                    time_despawn_pea,
                    despawn_zombie,
                ).run_if(in_state(GameState::Game)),
            )
            // Zombie Plant
            .add_event::<ZombieCollidePlantEvent>()
            .add_event::<PlantReceiveDamageEvent>()
            .add_event::<ZombieTargetNotExistEvent>()
            .add_event::<ZombiePoleJumpEvent>()
            .add_event::<ZombiePoleJump2Event>()
            .add_event::<ZombiePoleJumpEndEvent>()
            .add_systems(
                Update,
                (
                    detect_zombie_plant_collision,
                    handle_zombie_collide_plant,
                    zombie_attack_plant,
                    zombie_recover_walk_system,
                    zombie_pole_vaulting_recover_walk_system,
                    plant_receive_damage,
                ).run_if(in_state(GameState::Game)),
            )
            .add_systems(Update, play_zombie_animation.run_if(in_state(GameState::Game)))
            .add_systems(Update, spawn_pole_vaulting_animation_phase1.run_if(in_state(GameState::Game)))
            .add_systems(Update, spawn_pole_vaulting_animation_phase2.run_if(in_state(GameState::Game)))
            .add_systems(Update, play_pole_vaulting_jump1_animation.run_if(in_state(GameState::Game)))
            .add_systems(Update, play_pole_vaulting_jump2_animation.run_if(in_state(GameState::Game)))
            .add_systems(Update, spawn_pole_vaulting_zombie_walk.run_if(in_state(GameState::Game)))
            // 最后是关卡设计和游戏判定
            .insert_resource(Level::level1())
            .add_systems(Update, level_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, wave_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, zombie_arrive_room.run_if(in_state(GameState::Game)))
            ;
    }
}
