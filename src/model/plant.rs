use bevy::{platform::collections::HashMap, prelude::*};

use crate::config::PlantType;

/// 植物标记组件
#[derive(Component)]
pub struct Plant;

/// 植物血量
#[derive(Component)]
pub struct PlantHealth {
    /// 当前血量
    pub current: f32,
    /// 最大血量
    pub max: f32,
}

#[derive(Component)]
pub struct PeaShooter {
    pub damage: f32,
    pub fire_interval: Timer,
}
impl Default for PeaShooter {
    fn default() -> Self {
        Self {
            damage: 10.0,
            fire_interval: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
    
}

#[derive(Component)]
pub struct Sunflower {
    pub sun_amount: u32,
    pub interval: Timer,
}
impl Default for Sunflower {
    fn default() -> Self {
        Self {
            sun_amount: 25,
            interval: Timer::from_seconds(20., TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct WallNut;

#[derive(Resource, Deref, DerefMut)]
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