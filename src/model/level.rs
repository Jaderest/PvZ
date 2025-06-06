use bevy::prelude::*;

// 设计一个数据结构存储僵尸队列

use crate::model::zombie_events::*;

use std::collections::VecDeque;

#[derive(Component)]
pub struct ZombieWave {
    pub queue: Vec<ZombieType>,
    interval_timer: Timer,
}
impl ZombieWave {
    pub fn new(interval: f32) -> Self {
        ZombieWave {
            queue: Vec::new(),
            interval_timer: Timer::from_seconds(interval, TimerMode::Repeating),
        }
    }

    pub fn add_zombie(&mut self, zombie_type: ZombieType) {
        self.queue.push(zombie_type);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn pop_zombie(&mut self) -> Option<ZombieType> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Level (VecDeque<ZombieWave>);
impl Default for Level {
    fn default() -> Self {
        Level(VecDeque::new())
    }
}
impl Level {
    pub fn level1() -> Self {
        Level(VecDeque::new())
    }
}
