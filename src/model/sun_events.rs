use bevy::{prelude::*, scene::ron::de};

use super::sun::{Sun, SunPosition};

#[derive(Event, Debug)]
pub struct PickupSunEvent {
    pub amount: u32,
}

#[derive(Event, Debug)]
pub struct SpawnFlowerSunEvent {
    pub amount: u32,
    pub start: Vec3,
    pub end: Vec3,
    pub sun_position: SunPosition,
}

#[derive(Event, Debug, Deref, DerefMut)]
pub struct SunChangeEvent(pub u32);