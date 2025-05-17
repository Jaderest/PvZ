use bevy::prelude::*;

#[derive(Component)]
pub struct Sun {
    pub amount: u32,
}

#[derive(Component)]
pub struct SunPosition {
    pub x: f32,
    pub y: f32,
}
