use bevy::prelude::*;

#[derive(Component)]
pub struct Sun(pub u32);

#[derive(Component)]
pub struct SunPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SunAmount(u32);

impl Default for SunAmount {
    fn default() -> Self {
        Self(0)
    }
}

impl SunAmount {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }
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

// ========== UI ==========
#[derive(Component)]
pub struct SunUI;