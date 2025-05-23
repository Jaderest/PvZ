use bevy::prelude::*;

/// 豌豆标记组件
#[derive(Component)]
pub struct Pea;

/// 抛射物速度，可扩展为杨桃星星
#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ProjPosition {
    pub x: f32,
    pub y: u32,
}

#[derive(Component)]
pub struct ProjDamage {
    pub damage: f32,
}