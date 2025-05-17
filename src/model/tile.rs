use bevy::prelude::*;
use std::collections::HashMap;

use crate::model::components::GridPosition;

/// 地图块标记组件
#[derive(Component)]
pub struct Tile;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Lawn(pub HashMap<GridPosition, Entity>);

/// 地图块类型
#[derive(Component, Clone, Copy, Debug)]
pub enum TileType {
    /// 草地
    Grass,
    /// 土地
    Soil,
    /// 水
    Water,
}

#[derive(Component)]
pub struct Child {
    pub plant: Option<Entity>,
}

impl Child {
    pub fn new(plant: Option<Entity>) -> Self {
        Self { plant }
    }

    pub fn is_none(&self) -> bool {
        self.plant.is_none()
    }
}

impl Default for Child {
    fn default() -> Self {
        Self { plant: None }
    }
}
