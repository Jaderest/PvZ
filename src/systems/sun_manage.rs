use std::ops::Add;

use bevy::prelude::*;
use rand::rng;
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
            .add_systems(Startup, setup_sun_ui)
            .add_systems(Update, update_sun_ui)
            .add_systems(Update, sun_produce_sun)
            ;
    }
}

/// Sun UI
fn setup_sun_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Text::new("Sun: "),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        // TODO:　研究一下UI怎么用图片
        // Sprite {
        //     image: asset_server.load("Simple/Sun.png"),
        //     // Only specify fields you want to override, the rest will use default values
        //     ..default()
        // },
        SunUI,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.5, 0.5)),
        )],
    ));
}

fn update_sun_ui(
    sun_amount: Res<SunAmount>,
    sun_root: Single<Entity, (With<SunUI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*sun_root, 1) = sun_amount.to_string();
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
        //TODO：生成实体
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

fn sun_add(mut sun_amount: SunAmount, sun: u32) {
    sun_amount.add(sun);
}