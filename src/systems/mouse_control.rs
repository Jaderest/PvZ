use bevy::{prelude::*, render::camera};

use crate::config::GameConfig;
use crate::model::{components::GridPosition, tile::Tile};
use crate::model::plant_events::*;

// 检测点击事件
pub fn handle_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&camera::Camera, &GlobalTransform)>,
    tiles: Query<(&GridPosition, &Transform), With<Tile>>,
    game_config: Res<GameConfig>,
    mut spawn_plant_writer: EventWriter<SpawnPlantEvent>,
    mut despawn_plant_writer: EventWriter<DespawnPlantEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) && !mouse.just_pressed(MouseButton::Right) {
        return;
    }
    // if !mouse.just_pressed(MouseButton::Right) {
    //     return;
    // }
    let (camera, camera_transform) = camera_query.into_inner();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let world_position = camera
        .viewport_to_world(camera_transform, cursor_position)
        .unwrap()
        .origin;

    // info!("cursor_position: {:?}", cursor_position);
    // info!("world_pos: {:?}", world_position);

    let tile_size = game_config.tile_size;
    for (grid_position, transform) in tiles.iter() {
        let dx = (world_position.x - transform.translation.x).abs();
        let dy = (world_position.y - transform.translation.y).abs();
        if dx < tile_size / 2.0 && dy < tile_size / 2.0 {
            if mouse.just_pressed(MouseButton::Left) {
                // Spawn plant event
                spawn_plant_writer.write(SpawnPlantEvent {
                    grid_position: *grid_position,
                });
                info!("Clicked left on tile at position: {:?}", grid_position);
            } else if mouse.just_pressed(MouseButton::Right) {
                // Despawn plant event
                despawn_plant_writer.write(DespawnPlantEvent {
                    grid_position: *grid_position,
                });
                info!("Clicked right on tile at position: {:?}", grid_position);
            }
            // spawn_plant_writer.write(SpawnPlantEvent {
            //     grid_position: *grid_position,
            // });
        }
    }
}
