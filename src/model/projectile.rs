use bevy::prelude::*;

/// 豌豆标记组件
#[derive(Component)]
pub struct Pea;

/// 抛射物速度，可扩展为杨桃星星
#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Velocity {
    pub fn get_pea() -> Self {
        Self { x: 600., y: 0. }
    }
}

#[derive(Component)]
pub struct ProjRow(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct Hit(pub bool);
impl Default for Hit {
    fn default() -> Self {
        Self(false)
    }
}
impl Hit {
    pub fn is_hit(&self) -> bool {
        self.0
    }
    pub fn set_hit(&mut self, hit: bool) {
        self.0 = hit;
    }
}

#[derive(Component)]
pub struct ProjDamage(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct ProjLife(Timer);
impl Default for ProjLife {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}
