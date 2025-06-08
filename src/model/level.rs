use bevy::prelude::*;

// 设计一个数据结构存储僵尸队列

use crate::model::zombie_events::ZombieType::*;
use crate::model::zombie_events::*;

use std::collections::VecDeque;

// 其实我感觉最后还是要搓成一个实体
#[derive(Component)]
pub struct ZombieWave {
    pub queue: VecDeque<ZombieType>,
    interval_timer: Timer,
}
impl ZombieWave {
    pub fn new(interval: f32) -> Self {
        ZombieWave {
            queue: VecDeque::new(),
            interval_timer: Timer::from_seconds(interval, TimerMode::Repeating),
        }
    }
    pub fn add_zombies(&mut self, zombies: Vec<ZombieType>) {
        for zombie in zombies {
            self.queue.push_back(zombie);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn tick(&mut self, time: &Time) -> &Timer {
        self.interval_timer.tick(time.delta())
    }
}

#[derive(Resource)]
pub struct Level{
    waves: VecDeque<ZombieWave>,
    start_timer: Timer,
    interval_timer: Timer,
}
impl Default for Level {
    fn default() -> Self {
        Level {
            waves: VecDeque::new(),
            // 开始游戏到第一波僵尸
            start_timer: Timer::from_seconds(30.0, TimerMode::Once),
            // 波之间的最长间隔
            interval_timer: Timer::from_seconds(30.0, TimerMode::Repeating),
        }
    }
}

impl Level {
    pub fn level1() -> Self {
        let mut level = Level::default();
        let mut wave1 = ZombieWave::new(8.0);
        wave1.add_zombies(vec![Zombie, Zombie, Zombie]);
        level.push_back(wave1);
        let mut wave2 = ZombieWave::new(8.0);
        wave2.add_zombies(vec![Conehead, Zombie, Conehead, Zombie, Conehead]);
        level.push_back(wave2);
        let mut wave3 = ZombieWave::new(8.0);
        wave3.add_zombies(vec![PoleVaulting, Zombie, Conehead, Zombie, Zombie, Conehead, Zombie, Zombie]);
        level.push_back(wave3);

        level
    }

    fn push_back(&mut self, wave: ZombieWave) {
        self.waves.push_back(wave);
    }
    pub fn is_empty(&self) -> bool {
        self.waves.is_empty()
    }
    pub fn pop_front(&mut self) -> Option<ZombieWave> {
        self.waves.pop_front()
    }
    pub fn start_tick(&mut self, time: &Time) -> &Timer {
        self.start_timer.tick(time.delta())
    }
    pub fn is_started(&self) -> bool {
        self.start_timer.finished()
    }
    pub fn interval_tick(&mut self, time: &Time) -> &Timer {
        self.interval_timer.tick(time.delta())
    }
    pub fn reset_interval(&mut self) {
        self.interval_timer.reset();
    }
}
