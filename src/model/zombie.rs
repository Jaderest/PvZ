use bevy::prelude::*;

#[derive(Component)]
pub struct Zombie;

/// 属性之一：是否可以跳过第一个植物
#[derive(Component, Debug)]
pub struct ZombiePoleVaulting {
    pub can_jump: bool,
}
impl Default for ZombiePoleVaulting {
    fn default() -> Self {
        Self { can_jump: true }
    }
}
impl ZombiePoleVaulting {
    pub fn can_jump(&mut self) -> bool {
        self.can_jump
    }
    pub fn jump(&mut self) {
        self.can_jump = false;
    }
}

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
    current: f32,
    max: f32,
}
impl ZombieHealth {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }
    pub fn receive_damage(&mut self, damage: f32) -> bool {
        self.current -= damage;
        return self.current <= 0.0;
    }
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
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
impl Default for ZombieDefender {
    fn default() -> Self {
        Self::None
    }
}
impl ZombieDefender {
    pub fn get_defender(&mut self) -> Option<&mut Defender> {
        match self {
            Self::Some(defender) => Some(defender),
            Self::None => None,
        }
    }
    pub fn clear_defender(&mut self) {
        *self = Self::None;
    }
    pub fn conehead() -> Self {
        Self::Some(Defender::new_conehead())
    }
    pub fn normal() -> Self {
        Self::None
    }
    // 如果要扩展在游戏过程中为所有僵尸添加防具，那么可以在这里加
}

#[derive(Component, Debug)]
pub struct Defender {
    pub defender: DefenderType,
    pub health: f32,
}
impl Defender {
    pub fn new_conehead() -> Self {
        Self {
            defender: DefenderType::Cone,
            health: 100.0,
        }
    }
    pub fn receive_damage(&mut self, damage: f32) -> bool {
        self.health -= damage;
        return self.health <= 0.0;
    }
}

#[derive(Component, Debug)]
pub enum DefenderType {
    Cone,
    Bucket,
}
