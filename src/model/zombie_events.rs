use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ZombieSpawnEvent {
    pub y: u32,
    pub zombie_type: ZombieType,
}

#[derive(Debug, Clone, Copy)]
pub enum ZombieType {
    Zombie,
    Conehead,
    PoleVaulting,
}

#[derive(Event, Debug)]
pub struct ZombieDefenderBrokenEvent {
    pub zombie: Entity,
}