use bevy::{
    prelude::*,
};
use rand::Rng;

use crate::{model::sun::*, view::get_sprites::get_sun_sprite};
use crate::model::sun_events::*;
use crate::{config::*, model::plant_events::SuccessSpawnPlantEvent};

pub struct SunPlugin;
impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        // todo：注册生成阳光
        app.insert_resource(GlobalSunTimer::default())
            .insert_resource(SunAmount::default())
            .add_event::<PickupSunEvent>()
            .add_event::<SpawnFlowerSunEvent>()
            .add_event::<SunChangeEvent>()
            .add_systems(Update, sun_produce_sun)
            .add_systems(Update, sun_add)
            .add_systems(Update, sun_consume)
            .add_systems(Update, sun_despawn_with_time)
            .add_systems(Update, flower_produce_sun)
            .add_systems(Update, sun_fall_system)
            .add_systems(Update, flower_sun_fall_system)
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
        let x: f32 = rng.random_range(0.0..8.0);
        let y: f32 = rng.random_range(0.0..4.0);
        
        let sun_position = grid2pixel(*game_config, x, y, 10.0);
        let start_position = Vec3::new(sun_position.x, 500., sun_position.z);

        commands.spawn((
            get_sun_sprite(&asset_server),
            Sun(25),
            FallingSun,
            FallTimer::default(),
            SunDespawnTimer::default(),
            Transform {
                translation: start_position,
                scale: Vec3::splat(2.),
                ..default()
            },
            SunDrop {
                start: start_position,
                end: sun_position,
            },
        ));
    }
}

fn flower_produce_sun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sunflower_produce_reader: EventReader<SpawnFlowerSunEvent>,
) {
    for event in sunflower_produce_reader.read() {
        let start = event.start;
        let end = event.end;
        let amount = event.amount;

        commands.spawn((
            get_sun_sprite(&asset_server),
            Sun(amount),
            FlowerSun,
            FlowerSunTimer::default(),
            SunDespawnTimer::default(),
            Transform {
                translation: start,
                scale: Vec3::splat(2.),
                ..default()
            },
            FlowerSunDrop {
                start: start,
                end: end,
            },
        ));
    }
}

fn sun_add(
    mut sun_amount: ResMut<SunAmount>,
    mut pickup_sun_reader: EventReader<PickupSunEvent>,
    mut sun_change_writer: EventWriter<SunChangeEvent>,
) {
    for event in pickup_sun_reader.read() {
        sun_amount.add(event.amount);
        sun_change_writer.write(SunChangeEvent(sun_amount.get()));
    }
}

fn sun_consume(
    mut sun_amount: ResMut<SunAmount>,
    mut suc_spawn_plant_reader: EventReader<SuccessSpawnPlantEvent>,
    mut sun_change_writer: EventWriter<SunChangeEvent>,
) {
    for event in suc_spawn_plant_reader.read() {
        sun_amount.sub(event.sun_cost);
        sun_change_writer.write(SunChangeEvent(sun_amount.get()));
    }
}

fn sun_despawn_with_time(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SunDespawnTimer), With<Sun>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}


fn sun_fall_system(
    mut sun_query: Query<(&mut Transform, &mut FallTimer, &SunDrop), With<FallingSun>>,
    time: Res<Time>,
) {
    for (mut transform, mut timer, sun_drop) in sun_query.iter_mut() {
        timer.tick(time.delta());
        let t = timer.fraction();
        transform.translation = sun_drop.start.lerp(sun_drop.end, t);
    }
}

fn flower_sun_fall_system(
    mut sun_query: Query<(&mut Transform, &mut FlowerSunTimer, &FlowerSunDrop), With<FlowerSun>>,
    time: Res<Time>,
) {
    for (mut transform, mut timer, sun_drop) in sun_query.iter_mut() {
        timer.tick(time.delta());
        let t = timer.fraction();
        // transform.translation = sun_drop.start.lerp(sun_drop.end, t);
        let x_start = sun_drop.start.x;
        let y_start = sun_drop.start.y;
        let x_end = sun_drop.end.x;
        let z = sun_drop.start.z;

        let x_max = 1./2. * x_start + 1./2. * x_end;
        
        let a = -0.1;
        let b = -2. * a * x_max;
        let c = y_start - a * x_start * x_start - b * x_start;

        let x_pos = t * (x_end - x_start) + x_start;
        let y_pos = a * x_pos * x_pos + b * x_pos + c;
        
        transform.translation = Vec3::new(x_pos, y_pos, z);
    }
}