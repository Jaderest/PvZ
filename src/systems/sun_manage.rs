use bevy::prelude::*;

use crate::model::sun::{Sun, SunPosition};

// todo: 为天上生成阳光实现动画

struct SunManagePlugin;
impl Plugin for SunManagePlugin {
    fn build(&self, app: &mut App) {
        // todo：注册生成阳光
        app.insert_resource(GlobalSunTimer::default())
            .insert_resource(SunAmount(0))
            ;
    }
}

#[derive(Resource)]
struct GlobalSunTimer(Timer);
impl Default for GlobalSunTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
struct SunAmount(u32);

/// 自然生成阳光
fn sun_produce_sun(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GlobalSunTimer>,
    mut sun_amount: ResMut<SunAmount>,
) {
    
}