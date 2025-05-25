use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use rand::Rng;

use crate::model::{projectile::Velocity, zombie::*};
use crate::model::zombie_events::*;
use crate::model::{components::UiTimer, projectile::Hit};
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

pub struct ZombiePlugin;
impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ZombieSpawnEvent>()
            .add_systems(Update, spawn_zombie)
            .add_systems(Update, play_zombie_animation)
            .add_systems(Update, keyboard_spawn_zombie)
            .add_systems(Update, zombie_move)
            ;
    }
}

pub fn spawn_zombie(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_spawn_event: EventReader<ZombieSpawnEvent>,
) {
    for event in zombie_spawn_event.read() {
        info!("Spawn zombie at y: {}", event.y);
        let zombie_position = ZombiePosition::new(9.5, event.y);
        let mut zombie_translation = grid2pixel(
            *game_config,
            zombie_position.x,
            zombie_position.y as f32,
            7. - event.y as f32 * 0.1,
        );
        let zombie_speed = ZombieSpeed { speed: 18. };
        let zombie_damage = ZombieDamage { damage: 10.0 };

        //TODO: 检查魔法数字
        zombie_translation.y += 40.0;
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            commands.spawn((
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 0),
                Zombie,
                zombie_position,
                zombie_speed,
                ZombieAtkTimer::default(),
                ZombieHealth {
                    current: 100.0,
                    max: 100.0,
                },
                zombie_damage,
                ZombieDefender::None,
                UiTimer::zombie_type0(),
                Transform {
                    translation: zombie_translation,
                    scale: Vec3::splat(1.7),
                    ..default()
                },
            ));
        } else {
            commands.spawn((
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 1),
                Zombie,
                zombie_position,
                zombie_speed,
                ZombieAtkTimer::default(),
                ZombieHealth {
                    current: 100.0,
                    max: 100.0,
                },
                zombie_damage,
                ZombieDefender::None,
                UiTimer::zombie_type1(),
                Transform {
                    translation: zombie_translation,
                    scale: Vec3::splat(1.7),
                    ..default()
                },
            ));
        }
    }
}

pub fn zombie_move(
    mut zombie_query: Query<(&mut Transform, &ZombieSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, speed) in zombie_query.iter_mut() {
        transform.translation.x -= speed.speed * time.delta_secs();
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