use bevy::prelude::*;

pub fn get_sunflower_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(73, 74), 18, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("atlas/SunFlower.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_peashooter_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(71, 71), 12, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("atlas/PeaShooter.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_wallnut_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(71, 71), 16, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("atlas/WallNut.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}
