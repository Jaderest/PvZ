use crate::model::events::*;
use crate::model::zombie_events::*;
use crate::model::{components::UiTimer, projectile::Hit};
use crate::model::{projectile::Velocity, zombie::*};
use crate::systems::keyboard_control::*;
use crate::view::get_sprites::get_zombie_sprite;
use crate::view::play_animation::*;
use crate::{
    config::*,
    model::{
        projectile::{Pea, ProjDamage, ProjRow},
        zombie,
    },
};
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use rand::Rng;

pub fn spawn_zombie(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_spawn_event: EventReader<ZombieSpawnEvent>,
) {
    for event in zombie_spawn_event.read() {
        info!("Spawn zombie at y: {}", event.y);
        let zombie_position = ZombiePosition::new(8.5, event.y);
        let mut zombie_translation = grid2pixel(
            *game_config,
            zombie_position.x,
            zombie_position.y as f32,
            7. - event.y as f32 * 0.1,
        );
        let zombie_speed = ZombieSpeed { speed: 18. };
        let zombie_damage = ZombieDamage { damage: 10.0 };

        //TODO: 检查魔法数字
        zombie_translation.y += 60.0;
        let mut rng = rand::rng();
        let (zombie_sprite, zombie_type) = if rng.random_bool(0.5) {
            (
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 0),
                UiTimer::zombie_type0(),
            )
        } else {
            (
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 1),
                UiTimer::zombie_type1(),
            )
        };
        commands.spawn((
            zombie_sprite,
            Zombie,
            zombie_position,
            zombie_speed,
            ZombieBehavior::default(),
            ZombieAtkTimer::default(),
            ZombieTargetPlant::default(),
            ZombieHealth {
                current: 100.0,
                max: 100.0,
            },
            zombie_damage,
            ZombieDefender::None,
            zombie_type,
            Transform {
                translation: zombie_translation,
                scale: Vec3::splat(2.0),
                ..default()
            },
        ));
    }
}

pub fn zombie_move(
    mut zombie_query: Query<(&mut Transform, &ZombieSpeed, &ZombieBehavior), With<Zombie>>,
    time: Res<Time>,
) {
    for (mut transform, speed, behavior) in zombie_query.iter_mut() {
        if behavior.is_walk() {
            transform.translation.x -= speed.speed * time.delta_secs();
        }
    }
}

pub fn despawn_zombie(
    mut commands: Commands,
    mut zombie_query: Query<(Entity, &ZombieHealth, &Transform), With<Zombie>>,
) {
    for (entity, health, transform) in zombie_query.iter_mut() {
        if health.current <= 0.0 {
            info!("Zombie despawned at position: {:?}", transform.translation);
            commands.entity(entity).despawn();
            // 发送一个事件，用实体播放僵尸死亡动画
        }
    }
}

pub fn zombie_recover_walk_system(
    mut zombie_query: Query<(&mut ZombieBehavior, &mut ZombieTargetPlant), With<Zombie>>,
    mut zombie_target_not_exist_reader: EventReader<ZombieTargetNotExistEvent>,
) {
    for event in zombie_target_not_exist_reader.read() {
        if let Ok((mut zombie_behavior, mut zombie_target)) = zombie_query.get_mut(event.zombie) {
            if zombie_behavior.is_attack() {
                *zombie_behavior = ZombieBehavior::Walk;
                zombie_target.clear_target();
                info!(
                    "Zombie {} recovered to walk state, target not exists",
                    event.zombie
                );
            }
        } else {
            info!("Zombie {} not found for recovery", event.zombie);
        }
    }
}
