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

#[derive(Component, Debug)]
pub struct ZombieRunTimer {
    pub timer: Timer,
}
impl Default for ZombieRunTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Debug)]
pub struct ZombieAtkTimer {
    pub timer: Timer,
}
impl Default for ZombieAtkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
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
