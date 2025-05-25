use bevy::prelude::*;

#[derive(Component)]
pub struct Zombie;

#[derive(Component, Debug)]
pub struct ZombiePosition {
    pub x: f32,
    pub y: u32,
}
impl ZombiePosition {
    pub fn new(x: f32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug)]
pub struct ZombieSpeed {
    pub speed: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ZombieBehavior {
    Walk,
    Attack,
}
impl Default for ZombieBehavior {
    fn default() -> Self {
        Self::Walk
    }
}
impl ZombieBehavior {
    pub fn is_walk(&self) -> bool {
        matches!(self, Self::Walk)
    }
    pub fn is_attack(&self) -> bool {
        matches!(self, Self::Attack)
    }
    pub fn set_to(&mut self, behavior: Self) {
        // 这个Self表示自己的类型
        *self = behavior;
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct ZombieAtkTimer(Timer);
impl Default for ZombieAtkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Component, Debug)]
pub enum ZombieTargetPlant {
    Some(Entity),
    None,
}
impl Default for ZombieTargetPlant {
    fn default() -> Self {
        Self::None
    }
}
impl ZombieTargetPlant {
    pub fn get_target(&self) -> Option<Entity> {
        match self {
            Self::Some(entity) => Some(*entity),
            Self::None => None,
        }
    }
    pub fn set_target(&mut self, target: Entity) {
        *self = Self::Some(target);
    }
    pub fn clear_target(&mut self) {
        *self = Self::None;
    }
}

#[derive(Component, Debug)]
pub struct ZombieHealth {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Debug)]
pub struct ZombieDamage {
    pub damage: f32,
}

#[derive(Component, Debug)]
pub enum ZombieDefender {
    Some(Defender),
    None,
}

#[derive(Component, Debug)]
pub struct Defender {
    pub defender: DefenderType,
    pub health: f32,
}

#[derive(Component, Debug)]
pub enum DefenderType {
    Cone,
    Bucket,
}
