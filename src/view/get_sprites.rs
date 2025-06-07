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

pub fn get_sun_sprite(asset_server: &AssetServer) -> Sprite {
    Sprite {
        image: asset_server.load("other/Sun.png"),
        ..default()
    }
}

pub fn get_pea_sprite(asset_server: &AssetServer) -> Sprite {
    Sprite {
        image: asset_server.load("other/ProjectilePea.png"),
        ..default()
    }
}

pub fn get_zombie_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    sprite_type: u8,
) -> Sprite {
    match sprite_type {
        0 => return get_zombie_sprite_0(asset_server, texture_atlas_layouts),
        1 => return get_zombie_sprite_1(asset_server, texture_atlas_layouts),
        _ => panic!("Invalid zombie sprite type"),
    }
}

fn get_zombie_sprite_0(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(166, 144), 22, 1, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/Zombie.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

fn get_zombie_sprite_1(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(166, 144), 31, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/Zombie2.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_conehead_zombie_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(166, 144), 21, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/ConeheadZombie.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_zombie_attack_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(166, 144), 21, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/ZombieAttack.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_conehead_zombie_attack_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(166, 144), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/ConeheadZombieAttack.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_polevaulting_zombie_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(348, 218), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/PoleVaultingZombie.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_polevaulting_zombie_walk_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(348, 218), 24, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/PoleVaultingZombieWalk.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_polevaulting_zombie_jump_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(348, 218), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/PoleVaultingZombieJump.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_polevaulting_zombie_jump2_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(348, 218), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/PoleVaultingZombieJump2.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        ..default()
    }
}

pub fn get_polevaulting_zombie_attack_sprite(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(348, 218), 14, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    Sprite {
        image: asset_server.load("Zombies/PoleVaultingZombieAttack.png"),
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

pub fn get_sunbank_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("Cards/SunBank.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_shovel_bank_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("Cards/ShovelBank.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_shovel_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("Cards/Shovel.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_menu_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("other/Menu.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_game_win_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("other/WinBackground.png");
    ImageNode {
        image: image,
        ..default()
    }
}

pub fn get_game_lose_imagenode(asset_server: &AssetServer) -> ImageNode {
    let image = asset_server.load("other/LoseBackground.png");
    ImageNode {
        image: image,
        ..default()
    }
}