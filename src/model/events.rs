use bevy::prelude::*;

use crate::model::{
    plant::*,
    projectile::*,
    zombie::*,
};

#[derive(Event, Debug)]
pub struct PeaHitZombieEvent {
    pub pea: Entity,
    pub zombie: Entity,
    pub damage: f32,
}

#[derive(Event, Debug)]
pub struct ZombieCollidePlantEvent {
    pub zombie: Entity,
    pub plant: Entity,
    pub zombie_behavior: ZombieBehavior,
}

#[derive(Event, Debug)]
pub struct PlantReceiveDamageEvent {
    pub plant: Entity,
    pub damage: f32,
}

#[derive(Event, Debug)]
pub struct ZombieTargetNotExistEvent {
    pub zombie: Entity,
}