use bevy::prelude::*;

use crate::config::GameConfig;
use crate::model::components::GridPosition;
use crate::model::tile::*;

pub fn setup_lawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut lawn: ResMut<Lawn>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let image_handle = asset_server.load("Simple/Tiles.png");

    let tile_size: f32 = game_config.tile_size;
    let lawn_width: f32 = game_config.map_width as f32 * tile_size;

    let bottom_edge_of_tile: f32 = - tile_size * (game_config.map_height as f32 - 2.0);
    let left_edge_of_tile = 0.0 - lawn_width / 2.0;

    let offset_x = left_edge_of_tile + tile_size / 2.0;
    let offset_y = bottom_edge_of_tile + tile_size / 2.0;

    println!("offset_x: {}, offset_y: {}", offset_x, offset_y);
    println!("bottom_edge_of_tile: {}, left_edge_of_tile: {}", bottom_edge_of_tile, left_edge_of_tile);
    println!("lawn_width: {}, tile_size: {}", lawn_width, tile_size);

    for x in 0..game_config.map_width {
        for y in 0..game_config.map_height {
            let pos = GridPosition::new(x, y);
            let tile_position = Vec3::new(
                offset_x + (x as f32 * tile_size),
                offset_y + (y as f32 * tile_size),
                0.0,
            );
            
            let entity = commands
                .spawn((
                    Sprite {
                        image: image_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 0,
                        }),
                        ..default()
                    },
                    pos,
                    Tile,
                    TileType::Grass,
                    Transform {
                        translation: tile_position,
                        scale: Vec3::splat(tile_size / 64.0),
                        ..Default::default()
                    },
                    Child::new(None),
                ))
                .id();
            lawn.insert(pos, entity);
        }
    }
}
//TODO: 添加点击生成一个实体，根据grid获取tile实体，并且把tile实体的Option<Entity>设为生成的植物实体
