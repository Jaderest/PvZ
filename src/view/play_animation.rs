use bevy::prelude::*;

use crate::config::{grid2pixel, pixel2gridx, GameConfig};
use crate::model::components::UiTimer;
use crate::model::plant::{Plant};
use crate::model::zombie::*;
use crate::model::zombie_pole_vaulting::*;
use crate::view::get_sprites::*;

pub fn play_plant_animation(
    mut plant_query: Query<(&mut Sprite, &mut UiTimer), With<Plant>>,
    time: Res<Time>,
) {
    for (mut sprite, mut timer) in plant_query.iter_mut() {
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            if timer.timer.tick(time.delta()).just_finished() {
                timer.index = (timer.index + 1) % timer.max_index;
                texture_atlas.index = timer.index;
            }
        }
    }
}

pub fn play_zombie_animation(
    mut zombie_query: Query<(&mut Sprite, &mut UiTimer), With<Zombie>>,
    time: Res<Time>,
) {
    for (mut sprite, mut timer) in zombie_query.iter_mut() {
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            if timer.timer.tick(time.delta()).just_finished() {
                timer.index = (timer.index + 1) % timer.max_index;
                texture_atlas.index = timer.index;
            }
        }
    }
}

pub fn spawn_pole_vaulting_animation_phase1 (
    game_config: Res<GameConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut event_reader: EventReader<ZombiePoleJumpEvent>,
) {
    // 生成一个sprite实体，然后逐帧播放动画播放完就despawn
    let asset_server = asset_server.clone();
    for event in event_reader.read() {
        info!("Spawn Pole Vaulting Zombie Jump Animation at: {:?}", event.translation);
        commands.spawn((
            get_polevaulting_zombie_jump_sprite(&asset_server, &mut texture_atlas_layouts),
            UiTimer::zombie_polevaulting_jump1(),
            Transform {
                translation: event.translation,
                scale: Vec3::splat(1.8),
                ..default()
            },
            ZombieHealth::from_zombie_health(&event.health),
            ZombiePosition {
                y: event.y,
                x: pixel2gridx(*game_config, event.translation.x),
            },
            ZombiePoleVaultingAni1,
        ));
    }
}

pub fn spawn_pole_vaulting_animation_phase2 (
    game_config: Res<GameConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut event_reader: EventReader<ZombiePoleJump2Event>,
) {
    // 生成一个sprite实体，然后逐帧播放动画播放完就despawn
    let asset_server = asset_server.clone();
    for event in event_reader.read() {
        info!("Spawn Pole Vaulting Zombie Jump2 Animation at: {:?}", event.translation);
        commands.spawn((
            get_polevaulting_zombie_jump2_sprite(&asset_server, &mut texture_atlas_layouts),
            UiTimer::zombie_polevaulting_jump2(),
            Transform {
                translation: event.translation,
                scale: Vec3::splat(1.8),
                ..default()
            },
            ZombieHealth::from_zombie_health(&event.health),
            ZombiePosition {
                y: event.y,
                x: pixel2gridx(*game_config, event.translation.x),
            },
            ZombiePoleVaultingAni2,
        ));
    }
}

pub fn play_pole_vaulting_jump1_animation(
    mut commands: Commands,
    mut ani_query: Populated<(Entity, &mut UiTimer, &mut Sprite, &Transform, &ZombieHealth, &ZombiePosition), With<ZombiePoleVaultingAni1>>,
    mut event_writer: EventWriter<ZombiePoleJump2Event>,
    time: Res<Time>,
) {
    for (entity, mut timer, mut sprite, transform, health, position) in ani_query.iter_mut() {
        let t = timer.timer.tick(time.delta()).fraction();
        if timer.timer.finished() {
            commands.entity(entity).despawn();
            let mut translation = transform.translation;
            translation.x -= 210.0; // 假设跳跃后位置向左移动100个单位
            event_writer.write(ZombiePoleJump2Event {
                y: position.y,
                health: ZombieHealth::from_zombie_health(&health),
                translation: translation,
            });
            continue;
        }
        let index = (t.clamp(0.0, 1.0) * timer.max_index as f32).round() as usize;
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = index;
        }
    }
}

pub fn play_pole_vaulting_jump2_animation(
    mut commands: Commands,
    mut ani_query: Populated<(Entity, &mut UiTimer, &mut Sprite, &Transform, &ZombieHealth, &ZombiePosition), With<ZombiePoleVaultingAni2>>,
    mut event_writer: EventWriter<ZombiePoleJumpEndEvent>,
    time: Res<Time>,
) {
    for (entity, mut timer, mut sprite, transform, health, position) in ani_query.iter_mut() {
        let t = timer.timer.tick(time.delta()).fraction();
        if timer.timer.finished() {
            commands.entity(entity).despawn();
            let translation = transform.translation;
            event_writer.write(ZombiePoleJumpEndEvent {
                y: position.y,
                health: ZombieHealth::from_zombie_health(&health),
                translation: translation,
            });
            continue;
        }
        let index = (t.clamp(0.0, 1.0) * timer.max_index as f32).round() as usize;
        info!("pole2 index: {}", index);
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = index;
        }
    }
}


