use bevy::prelude::*;

use rand::rand_core::le;
use rand::Rng;

use crate::model::level::*;
use crate::model::zombie::*;
use crate::model::zombie_events::*;

// 成功逻辑 & 释放僵尸逻辑
pub fn level_system(
    mut commands: Commands,
    mut level: ResMut<Level>,
    time: Res<Time>,
    wave_query: Query<&ZombieWave>,
    zombie_query: Query<Entity, With<Zombie>>,
) {
    if !level.is_started() {
        level.start_tick(&time);
    }
    if !wave_query.is_empty() {
        return;
    }
    if level.is_empty() && zombie_query.is_empty() {
        // 胜利条件：level队列为空且场上没有僵尸
        // 这里是胜利了，但是把胜利事件交给其他系统处理
    } else if !level.is_empty() && zombie_query.is_empty() {
        // 如果level队列不为空但场上没有僵尸，释放下一波僵尸
        if let Some(wave) = level.pop_front() {
            add_wave(&mut commands, wave);
        }
        level.reset_interval();
    } else {
        // 僵尸不空那就计时，超过时间就释放下一波僵尸
        if level.interval_tick(&time).just_finished() {
            if let Some(wave) = level.pop_front() {
                add_wave(&mut commands, wave);
            }
        }
    }
}

fn add_wave(
    commands: &mut Commands,
    zombie_wave: ZombieWave, // 直接把它所有权借走
) {
    commands.spawn((
        zombie_wave,
    ));
}

pub fn wave_system(
    mut commands: Commands,
    zombie_wave_query: Single<(Entity, &mut ZombieWave)>,
    mut zombie_spawn_writer: EventWriter<ZombieSpawnEvent>,
    time: Res<Time>,
) {
    let (wave, mut zombie_wave) = zombie_wave_query.into_inner();
    if zombie_wave.is_empty() {
        commands.entity(wave).despawn();
        return;
        // 不知道这里要不要call一下level_system来检测下一步？
        // 或者使用事件call
        // 或者level_system解耦一下，但是也要call相应的释放下一步的部分
    }
    if zombie_wave.tick(&time).just_finished() {
        // 发送生成僵尸事件
        let zombie_type = zombie_wave.queue.pop_front().unwrap();
        let y = rand::rng().random_range(0..5); // 假设y轴范围是0到4
        zombie_spawn_writer.write(ZombieSpawnEvent {
            y: y as u32,
            zombie_type: zombie_type,
        });
    }
}

