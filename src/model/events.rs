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
