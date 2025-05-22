use bevy::prelude::*;

#[derive(Component)]
pub struct Sun(pub u32);

#[derive(Component, Debug)]
pub struct SunPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SunAmount(u32);

impl Default for SunAmount {
    fn default() -> Self {
        Self(1000)
    }
}

impl SunAmount {
    pub fn add(&mut self, amount: u32) {
        self.0 += amount;
    }
    pub fn sub(&mut self, amount: u32) {
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Sun amount is not enough: {} < {}", self.0, amount);
        }
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct GlobalSunTimer(Timer);
impl Default for GlobalSunTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct SunDespawnTimer(Timer);
impl Default for SunDespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

// ========== UI ==========
#[derive(Component)]
pub struct SunUI;
