use bevy::prelude::*;
use rand::Rng;

use crate::model::plant::*;
use crate::model::events::*;
use crate::model::sun::SunAmount;
use crate::model::sun_events::SpawnFlowerSunEvent;
use crate::model::tile::Lawn;
use crate::model::projectile_events::PeaSpawnEvent;

use crate::config::*;
use crate::model::components::*;
use crate::model::plant_events::*;
use crate::model::tile::{Child, Tile};
use crate::model::zombie::*;
use crate::view::get_sprites::*;

// 根据事件，读取全局状态来生成植物
pub fn spawn_plant(
    mut commands: Commands,
    lawn: ResMut<Lawn>,
    plant_type: Res<PlantType>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game_config: Res<GameConfig>,
    plant_cost: ResMut<PlantCost>,
    sun_amount: ResMut<SunAmount>,
    mut events: EventReader<SpawnPlantEvent>,
    mut tile_query: Query<(&Transform, &mut Child), With<Tile>>,
    mut suc_spawn_plant_writer: EventWriter<SuccessSpawnPlantEvent>,
    mut fail_spawn_plant_writer: EventWriter<FailedSpawnPlantEvent>,
) {
    for event in events.read() {
        let grid_position = event.grid_position;

        let Some((transform, mut child)) =
            get_tile_mut_if_valid(&lawn, grid_position, &mut tile_query)
        else {
            continue;
        };

        if !has_enough_sun(&plant_type, &plant_cost, &sun_amount) {
            info!("Not enough sun to spawn plant at: {:?}", grid_position);
            fail_spawn_plant_writer.write(FailedSpawnPlantEvent);
            continue;
        }

        info!("Spawning plant at: {:?}", grid_position);
        let plant_entity = spawn_plant_entity(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            &game_config,
            &plant_type,
            grid_position,
            transform,
        );

        child.plant = Some(plant_entity);

        // 成功事件
        let cost = plant_cost.get(&*plant_type).unwrap();
        suc_spawn_plant_writer.write(SuccessSpawnPlantEvent { sun_cost: *cost });
    }
}

// 辅助函数，获取tile
fn get_tile_mut_if_valid<'a>(
    lawn: &Lawn,
    grid_position: GridPosition,
    tile_query: &'a mut Query<(&Transform, &mut Child), With<Tile>>,
) -> Option<(&'a Transform, Mut<'a, Child>)> {
    let tile_entity = lawn.get(&grid_position)?;
    let Ok((transform, child)) = tile_query.get_mut(*tile_entity) else {
        return None;
    };
    if !child.is_none() {
        info!("Tile already has a plant at: {:?}", grid_position);
        return None;
    }
    Some((transform, child))
}

fn has_enough_sun(plant_type: &PlantType, plant_cost: &PlantCost, sun_amount: &SunAmount) -> bool {
    let cost = plant_cost.get(plant_type).unwrap_or(&9999);
    *cost <= sun_amount.get()
}

fn spawn_plant_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    game_config: &GameConfig,
    plant_type: &PlantType,
    grid_position: GridPosition,
    transform: &Transform,
) -> Entity {
    let mut translation = transform.translation;
    translation.z = 2 as f32 - 0.1 * grid_position.y() as f32;
    match plant_type {
        PlantType::PeaShooter => commands
            .spawn((
                get_peashooter_sprite(asset_server, texture_atlas_layouts),
                GridPosition::new(grid_position.x(), grid_position.y()),
                Transform {
                    translation: translation,
                    scale: Vec3::splat(game_config.tile_width / 64.0),
                    ..default()
                },
                PlantHealth {
                    current: 100.0,
                    max: 100.0,
                },
                PeaShooter::default(),
                Plant,
                UiTimer::new_plant(0.11, 11),
            ))
            .id(),

        PlantType::Sunflower => commands
            .spawn((
                get_sunflower_sprite(asset_server, texture_atlas_layouts),
                GridPosition::new(grid_position.x(), grid_position.y()),
                Transform {
                    translation: translation,
                    scale: Vec3::splat(game_config.tile_width / 64.0),
                    ..default()
                },
                PlantHealth {
                    current: 100.0,
                    max: 100.0,
                },
                Sunflower::default(),
                Plant,
                UiTimer::new_plant(0.08, 17),
            ))
            .id(),

        PlantType::WallNut => commands
            .spawn((
                get_wallnut_sprite(asset_server, texture_atlas_layouts),
                GridPosition::new(grid_position.x(), grid_position.y()),
                Transform {
                    translation: translation,
                    scale: Vec3::splat(game_config.tile_width / 64.0),
                    ..default()
                },
                PlantHealth {
                    current: 300.0,
                    max: 300.0,
                },
                WallNut,
                Plant,
                UiTimer::new_plant(0.08, 15),
            ))
            .id(),
    }
}

pub fn shovel_plant(
    lawn: ResMut<Lawn>,
    mut events: EventReader<ShovelPlantEvent>,
    mut tile_query: Query<&mut Child, With<Tile>>,
    mut plant_query: Query<&mut PlantHealth, With<Plant>>,
) {
    for event in events.read() {
        let grid_position = event.grid_position;
        if let Some(&tile_entity) = lawn.get(&grid_position) {
            if let Ok(mut child) = tile_query.get_mut(tile_entity) {
                // 根据全局状态生成植物实体
                // 未加child是否为None的判断

                let Some(plant_entity) = child.plant else {
                    info!("No plant to despawn at: {:?}", grid_position);
                    continue;
                };
                info!("shovel here");
                if let Ok(mut plant_health) = plant_query.get_mut(plant_entity) {
                    plant_health.current = -1.;
                }
                child.plant = None;
            }
        }
    }
}

pub fn plant_receive_damage(
    mut plant_query: Query<&mut PlantHealth, With<Plant>>,
    mut events_reader: EventReader<PlantReceiveDamageEvent>,
) {
    for event in events_reader.read() {
        if let Ok(mut plant_health) = plant_query.get_mut(event.plant) {
            plant_health.current -= event.damage;
            info!("Plant {:?} remainning health: {}", event.plant, plant_health.current);
        } else {
            info!("Plant {:?} not found for damage event.", event.plant);
        }
    }
}

pub fn despawn_plant(
    mut commands: Commands,
    mut plant_query: Query<(Entity, &mut PlantHealth), With<Plant>>,
) {
    for (entity, health) in plant_query.iter_mut() {
        if health.current <= 0.0 {
            info!("Despawning plant entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}

/// 向日葵计时，并生成阳光
pub fn sunflower_produce(
    game_config: Res<GameConfig>,
    time: Res<Time>,
    mut sunflower_query: Query<(&mut Sunflower, &GridPosition, &Transform)>,
    mut sunflower_produce_writer: EventWriter<SpawnFlowerSunEvent>,
) {
    for (mut sunflower, grid_position, transform) in sunflower_query.iter_mut() {
        let mut rng = rand::rng();
        let mut start = transform.translation;
        start.z = 10.;
        if sunflower.interval.tick(time.delta()).just_finished() {
            let delta_x: f32 = rng.random_range(0.2..0.3);
            let sun_position = grid2pixel(
                *game_config,
                grid_position.x() as f32 + delta_x,
                grid_position.y() as f32,
                10.,
            );
            sunflower_produce_writer.write(SpawnFlowerSunEvent {
                amount: sunflower.sun_amount,
                start: start,
                end: sun_position,
            });
        }
    }
}

pub fn peashooter_shoot(
    time: Res<Time>,
    mut peashooter_query: Query<(&mut PeaShooter, &GridPosition, &Transform), With<Plant>>,
    zombie_query: Query<&ZombiePosition>,
    mut spawn_pea_writer: EventWriter<PeaSpawnEvent>,
) {
    for (mut peashooter, grid_position, transform) in peashooter_query.iter_mut() {
        peashooter.fire_interval.tick(time.delta());
        // 不知道这里计时器的just_finished()一直记下去会发生什么，看看要不要换成Once并且手动刷新？
        for zombie_position in zombie_query.iter() {
            if zombie_position.y != grid_position.y() {
                continue; // 只射击同一行的僵尸
            }
            let zombie_x = zombie_position.x;
            let plant_x = grid_position.x() as f32; // 植物的x坐标
            let mut translation = transform.translation;
            translation.z = 5.0 - 0.01 * grid_position.y() as f32; // 确保植物在正确的z轴位置

            if peashooter.fire_interval.just_finished() && zombie_x > plant_x {
                // 如果植物的射击间隔刚好结束，并且僵尸在植物的右侧
                spawn_pea_writer.write(PeaSpawnEvent {
                    start: translation,
                    start_grid: *grid_position,
                    damage: peashooter.damage,
                });
                break; // 有僵尸就射击，不能重复射击
            }
        }
    }
}