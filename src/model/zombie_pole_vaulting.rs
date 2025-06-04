use bevy::prelude::*;

use crate::model::zombie::ZombieHealth;

// 一会添加一个system来处理僵尸跳杆的事件

#[derive(Event, Debug)]
pub struct ZombiePoleJumpEvent {
    pub y: u32, // 图片绘制的高度
    pub health: ZombieHealth,
    pub translation: Vec3, 
}

#[derive(Event, Debug)]
pub struct ZombiePoleJump2Event {
    pub y: u32, // 图片绘制的高度
    pub health: ZombieHealth,
    pub translation: Vec3, 
}

#[derive(Event, Debug)]
pub struct ZombiePoleJumpEndEvent {
    pub y: u32, // 图片绘制的高度
    pub health: ZombieHealth,
    pub translation: Vec3,
}