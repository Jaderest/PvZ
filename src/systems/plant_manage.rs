use bevy::platform::time;
use bevy::prelude::*;
use rand::Rng;

use crate::model::plant::PeaShooter;
use crate::model::plant::PlantCost;
use crate::model::plant::PlantHealth;
use crate::model::plant::Sunflower;
use crate::model::plant::WallNut;
use crate::model::sun;
use crate::model::sun::Sun;
use crate::model::tile::Lawn;

use crate::config::*;
use crate::model::components::GridPosition;
use crate::model::tile::{Child, Tile};
use crate::model::plant_events::*;

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        // 添加event
        // 添加系统
        app.insert_resource(PlantCost::default())
            .add_event::<SpawnPlantEvent>()
            .add_event::<DespawnPlantEvent>()
            .add_event::<SuccessSpawnPlantEvent>()
            .add_systems(Update, spawn_plant)
            .add_systems(Update, despawn_plant)
            .add_systems(Update, sunflower_produce)
            ;
    }
}

// 根据事件，读取全局状态来生成植物
fn spawn_plant(
    mut commands: Commands,
    lawn: ResMut<Lawn>,
    plant_type: Res<PlantType>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut events: EventReader<SpawnPlantEvent>,
    mut tile_query: Query<(&Transform, &mut Child), With<Tile>>,
) {
    // 获取GridPosition，并添加到lawn中
    let peashooter_image = asset_server.load("Simple/PeaShooter.png");
    let sunflower_image = asset_server.load("Simple/Sunflower.png");
    for event in events.read() {
        let grid_position = event.grid_position;
        if let Some(&tile_entity) = lawn.get(&grid_position) {
            if let Ok((transform, mut child)) = tile_query.get_mut(tile_entity) {
                // 根据全局状态生成植物实体
                // 未加child是否为None的判断
                if !child.is_none() {
                    continue;
                }

                info!("Spawn plant at: {:?}", grid_position);

                // 这里需要根据植物类型来判断阳光的消耗

                let plant_entity = match *plant_type {
                    PlantType::PeaShooter => commands
                        .spawn((
                            Sprite {
                                image: peashooter_image.clone(),
                                ..default()
                            },
                            GridPosition::new(grid_position.x(), grid_position.y()),
                            Transform {
                                translation: Vec3 {
                                    x: transform.translation.x,
                                    y: transform.translation.y,
                                    z: 1.0,
                                },
                                scale: Vec3::splat(game_config.tile_size / 64.0),
                                ..default()
                            },
                            PlantHealth {
                                current: 100.0,
                                max: 100.0,
                            },
                            PeaShooter {
                                damage: 10.0,
                                fire_interval: Timer::from_seconds(1.0, TimerMode::Repeating),
                            }
                        ))
                        .id(),
                    PlantType::Sunflower => commands
                        .spawn((
                            Sprite {
                                image: sunflower_image.clone(),
                                ..default()
                            },
                            GridPosition::new(grid_position.x(), grid_position.y()),
                            Transform {
                                translation: Vec3 {
                                    x: transform.translation.x,
                                    y: transform.translation.y,
                                    z: 1.0, // 植物实体的z轴位置，均设置为1.0
                                },
                                scale: Vec3::splat(game_config.tile_size / 64.0),
                                ..default()
                            },
                            PlantHealth {
                                current: 100.0,
                                max: 100.0,
                            },
                            Sunflower {
                                sun_amount: 25,
                                interval: Timer::from_seconds(10.0, TimerMode::Repeating),
                            },
                        ))
                        .id(),
                    PlantType::WallNut => commands
                        .spawn((
                            Sprite {
                                image: sunflower_image.clone(),
                                ..default()
                            },
                            GridPosition::new(grid_position.x(), grid_position.y()),
                            Transform {
                                translation: Vec3 {
                                    x: transform.translation.x,
                                    y: transform.translation.y,
                                    z: 1.0,
                                },
                                scale: Vec3::splat(game_config.tile_size / 64.0),
                                ..default()
                            },
                            PlantHealth {
                                current: 300.0,
                                max: 300.0,
                            },
                            WallNut,
                        ))
                        .id(),
                };
                // 更新地图块的子实体
                child.plant = Some(plant_entity);
            }
        }
    }
}

fn despawn_plant(
    mut commands: Commands,
    lawn: ResMut<Lawn>,
    mut events: EventReader<DespawnPlantEvent>,
    mut tile_query: Query<&mut Child, With<Tile>>,
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
                commands.entity(plant_entity).despawn();
                info!("Despawn plant at: {:?}", grid_position);

                child.plant = None;
            }
        }
    }
}

/// 植物计时，并生成阳光
fn sunflower_produce(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut sunflower_query: Query<(&mut Sunflower, &GridPosition, &Transform)>,
) {
    for (mut sunflower, grid_position, transform) in sunflower_query.iter_mut() {
        let mut rng = rand::rng();
        if sunflower.interval.tick(time.delta()).just_finished() {
            let delta_x: f32 = rng.random_range(-0.5..0.5);
            let delta_y: f32 = rng.random_range(-0.5..0.5);
            let sun_position = grid2pixel(
                *game_config,
                grid_position.x() as f32 + delta_x,
                grid_position.y() as f32 + delta_y,
                2.0,
            );
            commands.spawn((
                Sprite {
                    image: asset_server.load("Simple/Sun.png"),
                    ..default()
                },
                Sun(sunflower.sun_amount),
                Transform {
                    translation: sun_position,
                    scale: Vec3::splat(2.),
                    ..default()
                },
            ));
        }
    }
}