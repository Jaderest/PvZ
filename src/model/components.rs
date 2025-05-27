//! 组件即属性

use bevy::prelude::*;

/// 位置
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    x: u32,
    y: u32,
}

impl GridPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

#[derive(Component)]
pub struct UiTimer {
    pub timer: Timer,
    pub index: usize,
    pub max_index: usize,
}

impl Default for UiTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 0,
        }
    }
}
impl UiTimer {
    pub fn new_plant(duration: f32, max_index: usize) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            index: 0,
            max_index,
        }
    }
    pub fn zombie_type0() -> Self {
        Self {
            timer: Timer::from_seconds(0.135, TimerMode::Repeating),
            index: 0,
            max_index: 21,
        }
    }
    pub fn zombie_type1() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 30,
        }
    }
    pub fn zombie_attack() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 20,
        }
    }
    pub fn zombie_conehead() -> Self {
        Self {
            timer: Timer::from_seconds(0.175714, TimerMode::Repeating),
            index: 0,
            max_index: 20,
        }
    }
    pub fn zombie_conehead_attack() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 10,
        }
    }
    pub fn zombie_polevaulting_walk() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            index: 0,
            max_index: 24,
        }
    }
    pub fn zombie_polevaulting_run() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 10,
        }
    }
    pub fn zombie_polevaulting_jump1() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 9,
        }
    }
    pub fn zombie_polevaulting_jump2() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 6,
        }
    }
    pub fn zombie_pole_vaulting_attack() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            index: 0,
            max_index: 13,
        }
    }
}
