use bevy::prelude::*;

pub fn get_sunflower_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(73, 74), 18, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Plants/SunFlower.png"),
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
        image: asset_server.load("Plants/PeaShooter.png"),
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
    let layout = TextureAtlasLayout::from_grid(UVec2::new(65, 73), 16, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Plants/WallNut.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_sunflower_card_imagenode(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> ImageNode {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 60), 1, 2, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    let image = asset_server.load("Cards/SunFlower.png");
    ImageNode::from_atlas_image(image, TextureAtlas::from(texture_atlas_handle))
}

pub fn get_peashooter_card_imagenode(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> ImageNode {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 60), 1, 2, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    let image = asset_server.load("Cards/PeaShooter.png");
    ImageNode::from_atlas_image(image, TextureAtlas::from(texture_atlas_handle))
}

pub fn get_wallnut_card_imagenode(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> ImageNode {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 60), 1, 2, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    let image = asset_server.load("Cards/WallNut.png");
    ImageNode::from_atlas_image(image, TextureAtlas::from(texture_atlas_handle))
}

pub fn get_sunbank_imagenode(
    asset_server: &AssetServer
) -> ImageNode {
    let image = asset_server.load("Cards/SunBank.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_shovel_bank_imagenode(
    asset_server: &AssetServer
) -> ImageNode {
    let image = asset_server.load("Cards/ShovelBank.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_shovel_imagenode(
    asset_server: &AssetServer
) -> ImageNode {
    let image = asset_server.load("Cards/Shovel.png");
    ImageNode {
        image: image,
        ..default()
    }
}