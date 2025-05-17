use bevy::prelude::*;

/// 植物标记组件
#[derive(Component)]
pub struct Plant;

/// 植物血量
#[derive(Component)]
pub struct PlantHealth {
    /// 当前血量
    current: f32,
    /// 最大血量
    max: f32,
}

#[derive(Component)]
pub struct PeaShooter {
    damage: f32,
    fire_interval: Timer,
}

#[derive(Component)]
pub struct Sunflower {
    sun_amount: u32,
    interval: Timer,
}

#[derive(Component)]
pub struct WallNut;