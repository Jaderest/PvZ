use bevy::{platform::collections::HashMap, prelude::*};

use crate::config::PlantType;

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

#[derive(Resource)]
pub struct PlantCost(pub HashMap<PlantType, u32>);

impl Default for PlantCost {
    fn default() -> Self {
        let mut cost = HashMap::new();
        cost.insert(PlantType::PeaShooter, 100);
        cost.insert(PlantType::Sunflower, 50);
        cost.insert(PlantType::WallNut, 50);
        Self(cost)
    }
}