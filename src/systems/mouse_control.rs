use bevy::ecs::system::command;
use bevy::state::commands;
use bevy::{prelude::*, render::camera};

use crate::config::*;
use crate::model::plant_events::*;
use crate::model::sun::Sun;
use crate::model::sun_events::*;
use crate::model::{components::GridPosition, tile::Tile};

// 检测点击事件
pub fn handle_clicks(
    commands: Commands,
    game_config: Res<GameConfig>,
    mut control_state: ResMut<ControlState>,
    plant_type: Res<PlantType>,

    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&camera::Camera, &GlobalTransform)>,

    tiles: Query<(&GridPosition, &Transform), With<Tile>>,
    spawn_plant_writer: EventWriter<SpawnPlantEvent>,
    despawn_plant_writer: EventWriter<DespawnPlantEvent>,

    sun: Query<(Entity, &Sun, &Transform), With<Sun>>,
    pickup_sun_writer: EventWriter<PickupSunEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) && !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    /* 简单写个状态切换 */
    if mouse.just_pressed(MouseButton::Right) {
        match *control_state {
            ControlState::Normal => {
                *control_state = ControlState::SelectPlant;
                info!("Switch to SelectPlant");
            }
            ControlState::SelectPlant => {
                *control_state = ControlState::Shovel;
                info!("Switch to Shovel");
            }
            ControlState::Shovel => {
                *control_state = ControlState::Normal;
                info!("Switch to Normal");
            }
        }
        return;
    }

    let (camera, camera_transform) = camera_query.into_inner();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let world_position = camera
        .viewport_to_world(camera_transform, cursor_position)
        .unwrap()
        .origin;


    // todo: 根据全全局状态判断调用哪个click
    // 写状态切换
    // 写向日葵 & 植物ui（学习一下Sprite UI）
    // 写阳光运动逻辑
    // 写僵尸生成
    // 僵尸运动逻辑
    // 写植物攻击逻辑
    // 写僵尸攻击逻辑
    match *control_state {
        ControlState::Normal => {
            // 处理植物点击事件
            info!("Normal click at: {:?}", world_position);
            // 这里需要判断是UI还是地图中
            sun_click(
                commands,
                *game_config,
                world_position,
                sun,
                mouse.clone(),
                pickup_sun_writer,
            );
        }
        _ => {
            // 处理植物点击事件
            info!("SelectPlant click at: {:?}", world_position);
            plant_click(
                *game_config,
                control_state,
                tiles,
                world_position,
                spawn_plant_writer,
                despawn_plant_writer,
            );
        }
    }
}

/// 处理传给植物的点击事件
fn plant_click(
    game_config: GameConfig,
    control_state: ResMut<ControlState>,
    tiles: Query<(&GridPosition, &Transform), With<Tile>>,

    click_world_position: Vec3,
    mut spawn_plant_writer: EventWriter<SpawnPlantEvent>,
    mut despawn_plant_writer: EventWriter<DespawnPlantEvent>,
) {
    let tile_size = game_config.tile_size;
    for (grid_position, transform) in tiles.iter() {
        let dx = (click_world_position.x - transform.translation.x).abs();
        let dy = (click_world_position.y - transform.translation.y).abs();
        if dx < tile_size / 2.0 && dy < tile_size / 2.0 {
            match *control_state {
                ControlState::SelectPlant => {
                    // 处理植物点击事件
                    spawn_plant_writer.write(SpawnPlantEvent {
                        grid_position: *grid_position,
                    });
                    info!("SelectPlant click at: {:?}", grid_position);
                }
                ControlState::Shovel => {
                    // 处理铲子点击事件
                    despawn_plant_writer.write(DespawnPlantEvent {
                        grid_position: *grid_position,
                    });
                    info!("SelectShovel click at: {:?}", grid_position);
                }
                _ => {
                    panic!("Invalid control state");
                }
            }
        }
    }
}

fn sun_click(
    mut commands: Commands,
    game_config: GameConfig,
    click_world_position: Vec3,
    suns: Query<(Entity, &Sun, &Transform), With<Sun>>,
    mouse: ButtonInput<MouseButton>,
    mut pickup_sun_writer: EventWriter<PickupSunEvent>,
) {
    let sun_size = game_config.sun_size;
    for (sun_entity, sun, transform) in suns.iter() {
        let dx = (click_world_position.x - transform.translation.x).abs();
        let dy = (click_world_position.y - transform.translation.y).abs();
        if dx < sun_size / 2.0 && dy < sun_size / 2.0 {
            if mouse.just_pressed(MouseButton::Left) {
                // Pickup sun event
                info!(
                    "Clicked left on sun at position: {:?}",
                    transform.translation
                );
                pickup_sun_writer.write(PickupSunEvent { amount: sun.0 });
                commands.entity(sun_entity).despawn();
                break;
            }
        }
    }
}
