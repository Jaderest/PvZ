use bevy::prelude::*;

use crate::config::*;
use crate::model::components::GridPosition;
use crate::model::tile::*;

// 设置一个草坪系统，辅助植物系统
// 可以扩展为其他类型的地块，比如水池、空地（不能放置植物）等
pub fn setup_lawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut lawn: ResMut<Lawn>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let image_handle = asset_server.load("Map/Tiles.png");

    let tile_height = game_config.tile_height;
    let tile_width = game_config.tile_width;

    for x in 0..game_config.map_width {
        for y in 0..game_config.map_height {
            let pos = GridPosition::new(x, y);
            let tile_position = grid2pixel(*game_config, x as f32, y as f32, 0.0);
            
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
                        scale: Vec3::new(
                            tile_width / 64.0,
                            tile_height / 64.0,
                            1.0,
                        ),
                        ..default()
                    },
                    Child::new(None),
                ))
                .id();
            lawn.insert(pos, entity);
        }
    }
}
