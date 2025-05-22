use bevy::prelude::*;

use crate::config::*;
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
    let image_handle = asset_server.load("Map/Tiles.png");

    let tile_size = game_config.tile_size;

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
                        scale: Vec3::splat(tile_size / 64.0),
                        ..default()
                    },
                    Child::new(None),
                ))
                .id();
            lawn.insert(pos, entity);
        }
    }
}
