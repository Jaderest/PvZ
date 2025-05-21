use bevy::prelude::*;
use rand::Rng;

use crate::config::*;
use crate::model::sun;
use crate::model::sun::*;
use crate::model::sun_events;
use crate::model::sun_events::*;

// todo: 为天上生成阳光实现动画

pub struct SunPlugin;
impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        // todo：注册生成阳光
        app.insert_resource(GlobalSunTimer::default())
            .insert_resource(SunAmount::default())
            .add_event::<PickupSunEvent>()
            .add_systems(Update, sun_produce_sun)
            .add_systems(Update, sun_add)
            ;
    }
}

/// 自然生成阳光
fn sun_produce_sun(
    // mut sun_amount: ResMut<SunAmount>,
    time: Res<Time>,
    game_config: Res<GameConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<GlobalSunTimer>,
) {
    let mut rng = rand::rng();
    timer.tick(time.delta());
    if timer.just_finished() {
        // sun_amount.add(25);
        // info!("Sun amount: {}", sun_amount.get());
        let x: f32 = rng.random_range(0.0..8.0);
        let y: f32 = rng.random_range(0.0..4.0);
        let sun_position = grid2pixel(*game_config, x, y, 2.0);
        commands.spawn((
            Sprite {
                image: asset_server.load("Simple/Sun.png"),
                ..default()
            },
            Sun(25),
            SunPosition { x, y },
            Transform {
                translation: sun_position,
                scale: Vec3::splat(2.),
                ..default()
            },
        ));
    }
}

fn sun_add(mut sun_amount: ResMut<SunAmount>, mut pickup_sun_reader: EventReader<PickupSunEvent>) {
    for event in pickup_sun_reader.read() {
        sun_amount.add(event.amount);
        info!("Sun amount: {}", sun_amount.get());
    }
}

// fn sun_consume(mut sun_amount: SunAmount, sun: u32) {
//     sun_amount.sub(sun);
// }