use bevy::prelude::*;

pub fn setup_game_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bg_img = asset_server.load("Map/DayBackground.png");
    commands.spawn((
        Sprite {
            image: bg_img,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -1.0), // 背景图层在最底层
            scale: Vec3::splat(1.6), // 800 / 600 = 1.333333
            ..default()
        },
    ));
}