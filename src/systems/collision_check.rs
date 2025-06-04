use bevy::{
    ecs::event,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    transform,
};

use crate::model::components::*;
use crate::model::projectile::*;
use crate::model::zombie::*;
use crate::model::zombie_pole_vaulting::*;
use crate::model::{events::*, zombie};
use crate::{
    config::GameConfig, model::zombie_events::ZombieDefenderBrokenEvent, view::get_sprites::*,
};
use crate::{config::pixel2gridx, model::plant::*};

pub fn detect_pea_zombie_collision(
    mut pea_query: Query<(Entity, &ProjDamage, &Transform, &ProjRow, &mut Hit), With<Pea>>,
    mut zombie_query: Query<(Entity, &Transform, &ZombiePosition), With<Zombie>>,
    mut collision_event_writer: EventWriter<PeaHitZombieEvent>,
) {
    for (pea_entity, damage, pea_transform, pea_row, mut hit) in pea_query.iter_mut() {
        for (zombie_entity, zombie_transform, zombie_position) in zombie_query.iter_mut() {
            if hit.is_hit() {
                continue; // Skip if the pea has already hit something
            }
            if zombie_position.y != pea_row.0 {
                continue;
            }
            let mut zombie_center = zombie_transform.translation.truncate();
            zombie_center.x += 50.0;
            let zombie_aabb = Aabb2d::new(zombie_center, Vec2::new(85.0 / 2.0, 129.0 / 2.0));

            let pea_circle = BoundingCircle::new(pea_transform.translation.truncate(), 28.0);

            if pea_collision(pea_circle, zombie_aabb) {
                collision_event_writer.write(PeaHitZombieEvent {
                    pea: pea_entity,
                    zombie: zombie_entity,
                    damage: damage.0,
                });
                hit.set_hit(true);
                break;
            }
        }
    }
}

fn pea_collision(pea: BoundingCircle, zombie: Aabb2d) -> bool {
    pea.intersects(&zombie)
}

pub fn handle_pea_hit_zombie(
    mut commands: Commands,
    mut events_reader: EventReader<PeaHitZombieEvent>,
    mut defender_break_events_writer: EventWriter<ZombieDefenderBrokenEvent>,
    mut zombie_query: Query<(&mut ZombieHealth, &mut ZombieDefender), With<Zombie>>,
) {
    for event in events_reader.read() {
        if let Ok((mut zombie_health, mut zombie_defender)) = zombie_query.get_mut(event.zombie) {
            if let Some(defender) = zombie_defender.get_defender() {
                if defender.receive_damage(event.damage) {
                    defender_break_events_writer.write(ZombieDefenderBrokenEvent {
                        zombie: event.zombie,
                    });
                }
            } else {
                zombie_health.receive_damage(event.damage);
            }
        }
        commands.entity(event.pea).despawn();

        // 这里可以添加更多处理，比如播放音效，生成爆炸粒子
    }
}

pub fn detect_zombie_plant_collision(
    game_config: Res<GameConfig>,
    mut plant_query: Query<(Entity, &Transform, &GridPosition), With<Plant>>,
    mut zombie_query: Query<(Entity, &Transform, &ZombiePosition, &ZombieBehavior), With<Zombie>>,
    mut collision_event_writer: EventWriter<ZombieCollidePlantEvent>,
) {
    for (plant_entity, plant_transform, plant_grid) in plant_query.iter_mut() {
        for (zombie_entity, zombie_transform, zombie_position, zombie_behavior) in
            zombie_query.iter_mut()
        {
            if zombie_position.y != plant_grid.y() {
                continue; // Skip if the zombie is not on the same row as the plant
            }
            if !zombie_behavior.is_walk() {
                continue; // Skip if the zombie is not in walking state
            }
            let mut zombie_center = zombie_transform.translation.truncate();
            zombie_center.x += 50.0;
            let zombie_aabb = Aabb2d::new(zombie_center, Vec2::new(85.0 / 4.0, 129.0 / 4.0));
            let zombie_bounding_circle = zombie_aabb.bounding_circle();

            let plant_center = plant_transform.translation.truncate();
            let plant_aabb = Aabb2d::new(plant_center, Vec2::splat(game_config.tile_size / 3.0));
            let plant_bounding_circle = plant_aabb.bounding_circle();
            if plant_bounding_circle.intersects(&zombie_bounding_circle) {
                collision_event_writer.write(ZombieCollidePlantEvent {
                    zombie: zombie_entity,
                    plant: plant_entity,
                    zombie_behavior: ZombieBehavior::Attack,
                });
            }
        }
    }
}

// 处理僵尸与植物碰撞事件
// 僵尸在碰撞到植物时，切换状态为攻击状态，并设置目标植物
pub fn handle_zombie_collide_plant(
    mut commands: Commands,
    mut events_reader: EventReader<ZombieCollidePlantEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_query: Query<
        (
            &mut ZombieBehavior,
            &mut ZombieAtkTimer,
            &mut ZombieTargetPlant,
            &mut ZombieDefender,
        ),
        (With<Zombie>, Without<ZombiePoleVaulting>),
    >,
) {
    for event in events_reader.read() {
        if let Ok((
            mut zombie_behavior,
            mut zombie_atk_timer,
            mut zombie_target,
            mut zombie_defender,
        )) = zombie_query.get_mut(event.zombie)
        {
            if zombie_behavior.is_walk() {
                zombie_behavior.set_to(event.zombie_behavior);
                zombie_target.set_target(event.plant);
                zombie_atk_timer.reset(); // 重置攻击计时器
                // 切换贴图和uitimer
                if let Some(_defender) = zombie_defender.get_defender() {
                    // 更新贴图
                    commands
                        .entity(event.zombie)
                        .insert(get_conehead_zombie_attack_sprite(
                            &asset_server,
                            &mut texture_atlas_layouts,
                        ));
                    commands
                        .entity(event.zombie)
                        .insert(UiTimer::zombie_conehead_attack());
                } else {
                    // 更新贴图
                    commands
                        .entity(event.zombie)
                        .insert(get_zombie_attack_sprite(
                            &asset_server,
                            &mut texture_atlas_layouts,
                        ));
                    commands
                        .entity(event.zombie)
                        .insert(UiTimer::zombie_attack());
                }
                info!(
                    "Zombie {} changes behavior to {:?} for plant {}",
                    event.zombie, zombie_behavior, event.plant
                );
            }
        }
    }
}

pub fn handle_pole_vaulting_zombie_collide_plant(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut events_reader: EventReader<ZombieCollidePlantEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_query: Populated<
        (
            &mut ZombieBehavior,
            &mut ZombieAtkTimer,
            &mut ZombieTargetPlant,
            &mut ZombiePoleVaulting,
            &mut ZombiePosition,
            &ZombieHealth,
            &Transform,
        ),
        With<ZombiePoleVaulting>,
    >,
    mut zombie_pole_vaulting_jump_event_writer: EventWriter<ZombiePoleJumpEvent>,
) {
    for event in events_reader.read() {
        if let Ok((
            mut zombie_behavior,
            mut zombie_atk_timer,
            mut zombie_target,
            mut zombie_pole_vaulting,
            zombie_position,
            zombie_health,
            zombie_transform,
        )) = zombie_query.get_mut(event.zombie)
        {
            if zombie_pole_vaulting.can_jump() {
                // 获取僵尸实体的位置
                let grid_x = pixel2gridx(*game_config, zombie_position.x);
                info!(
                    "pole vaulting zombie {:?} jumps at grid_x: {}, y: {}",
                    event.zombie, grid_x, zombie_transform.translation.y
                );
                zombie_pole_vaulting_jump_event_writer.write(ZombiePoleJumpEvent {
                    y: zombie_position.y,
                    health: zombie_health.clone(),
                    translation: zombie_transform.translation,
                });
                commands.entity(event.zombie).despawn();

                continue;
            }
            if zombie_behavior.is_walk() {
                info!(
                    "Pole Vaulting Zombie {:?} collides with plant {:?}",
                    event.zombie, event.plant
                );
                zombie_behavior.set_to(event.zombie_behavior);
                zombie_target.set_target(event.plant);
                zombie_atk_timer.reset(); // 重置攻击计时器
                
                // 切换贴图和uitimer
                commands
                    .entity(event.zombie)
                    .insert(get_polevaulting_zombie_attack_sprite(
                        &asset_server,
                        &mut texture_atlas_layouts,
                    ));
                commands
                    .entity(event.zombie)
                    .insert(UiTimer::zombie_pole_vaulting_attack());
            }
        }
    }
}

pub fn zombie_attack_plant(
    mut zombie_query: Query<
        (
            Entity,
            &mut ZombieBehavior,
            &mut ZombieAtkTimer,
            &mut ZombieTargetPlant,
            &ZombieDamage,
        ),
        With<Zombie>,
    >,
    plant_query: Query<Entity, With<Plant>>,
    time: Res<Time>,
    mut plant_receive_dmg_event_writer: EventWriter<PlantReceiveDamageEvent>,
    mut zombie_target_not_exist_event_writer: EventWriter<ZombieTargetNotExistEvent>,
) {
    for (zombie_entity, zombie_behavior, mut zombie_atk_timer, zombie_target, zombie_damage) in
        zombie_query.iter_mut()
    {
        if zombie_behavior.is_attack() {
            if let Some(plant_entity) = zombie_target.get_target() {
                // 检查植物是否存在
                if let Ok(_) = plant_query.get(plant_entity) {
                    // 如果僵尸处于攻击状态，开始计时
                    zombie_atk_timer.tick(time.delta());
                    if zombie_atk_timer.just_finished() {
                        // 发送植物受到伤害的事件
                        plant_receive_dmg_event_writer.write(PlantReceiveDamageEvent {
                            plant: plant_entity,
                            damage: zombie_damage.damage,
                        });
                        info!(
                            "Zombie {:?} attacks plant {:?} for {} damage",
                            zombie_behavior, plant_entity, zombie_damage.damage
                        );
                    }
                } else {
                    zombie_target_not_exist_event_writer.write(ZombieTargetNotExistEvent {
                        zombie: zombie_entity,
                    });
                }
            } else {
                // 如果没有目标，恢复行走状态
                panic!(
                    "Zombie {:?} has no target plant, resetting behavior to Walk",
                    zombie_behavior
                );
            }
        }
    }
}
