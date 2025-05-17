use bevy::prelude::*;

use crate::model::tile::Lawn;

use crate::config::GameConfig;
use crate::config::PlantType;
use crate::model::components::GridPosition;
use crate::model::tile::{Child, Tile};
use crate::systems::plant_events::*;

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        // 添加event
        // 添加系统
        app.add_event::<SpawnPlantEvent>()
            .add_event::<DespawnPlantEvent>()
            .add_systems(Update, spawn_plant)
            .add_systems(Update, despawn_plant);
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
                        ))
                        .id(),
                    _ => commands
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
                                    z: 1.0, // 植物实体的z轴位置，均设置为1.0
                                },
                                scale: Vec3::splat(game_config.tile_size / 64.0),
                                ..default()
                            },
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
    plant_type: Res<PlantType>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
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
