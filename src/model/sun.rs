use bevy::prelude::*;

#[derive(Component)]
pub struct Sun(pub u32);

#[derive(Component)]
pub struct FallingSun;

#[derive(Component, Deref, DerefMut)]
pub struct FallTimer(Timer);
impl Default for FallTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.5, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct SunDrop {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Component)]
pub struct FlowerSun;

#[derive(Component)]
pub struct FlowerSunDrop {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Component, Deref, DerefMut)]
pub struct FlowerSunTimer(Timer);
impl Default for FlowerSunTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

// #[derive(Component, Debug)]
// pub struct SunPosition {
//     pub x: f32,
//     pub y: f32,
// }

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
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

// ========== UI ==========
#[derive(Component)]
pub struct SunUI;
