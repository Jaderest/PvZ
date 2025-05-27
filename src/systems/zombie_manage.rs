use crate::config::*;
use crate::model::components::UiTimer;
use crate::model::events::*;
use crate::model::zombie::*;
use crate::model::zombie_events::*;
use crate::view::get_sprites::get_conehead_zombie_sprite;
use crate::view::get_sprites::get_zombie_attack_sprite;
use crate::view::get_sprites::get_zombie_sprite;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_zombie(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_spawn_event: EventReader<ZombieSpawnEvent>,
) {
    for event in zombie_spawn_event.read() {
        match event.zombie_type {
            ZombieType::Zombie => {
                spawn_zombie_entity(
                    event.y,
                    &mut commands,
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &game_config,
                );
            }
            ZombieType::Conehead => {
                spawn_conehead_zombie_entity(
                    event.y,
                    &mut commands,
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &game_config,
                );
            }
            ZombieType::PoleVaulting => {}
        }
    }
}

fn spawn_zombie_entity(
    y: u32,
    commands: &mut Commands,
    asset_server: &AssetServer,
    mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    game_config: &GameConfig,
) {
    info!("Spawn zombie at y: {}", y);
    let zombie_position = ZombiePosition::new(8.5, y);
    let mut zombie_translation = grid2pixel(
        *game_config,
        zombie_position.x,
        zombie_position.y as f32,
        7. - y as f32 * 0.1,
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
        ZombieHealth::new(100.0),
        zombie_damage,
        ZombieDefender::normal(),
        zombie_type,
        Transform {
            translation: zombie_translation,
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
}

fn spawn_conehead_zombie_entity(
    y: u32,
    commands: &mut Commands,
    asset_server: &AssetServer,
    mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    game_config: &GameConfig,
) {
    let zombie_position = ZombiePosition::new(8.5, y);
    let mut zombie_translation = grid2pixel(
        *game_config,
        zombie_position.x,
        zombie_position.y as f32,
        7. - y as f32 * 0.1,
    );
    zombie_translation.y += 60.0;

    commands.spawn((
        get_conehead_zombie_sprite(&asset_server, &mut texture_atlas_layouts),
        Zombie,
        zombie_position,
        ZombieSpeed { speed: 18. },
        ZombieBehavior::default(),
        ZombieAtkTimer::default(),
        ZombieTargetPlant::default(),
        ZombieHealth::new(100.0),
        ZombieDamage { damage: 10.0 },
        ZombieDefender::conehead(),
        UiTimer::zombie_conehead(),
        Transform {
            translation: zombie_translation,
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
}

// fn spawn_pole_vaulting_zombie_entity(
//     y: u32,
//     commands: &mut Commands,
//     asset_server: &AssetServer,
//     mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
//     game_config: &GameConfig,
// ) {
//     let zombie_position = ZombiePosition::new(8.5, y);
//     let mut zombie_translation = grid2pixel(
//         *game_config,
//         zombie_position.x,
//         zombie_position.y as f32,
//         7. - y as f32 * 0.1,
//     );
//     zombie_translation.y += 60.0;

//     commands.spawn((
//         get_polevaulting_zombie_sprite(&asset_server, &mut texture_atlas_layouts),
//         Zombie,
//         zombie_position,
//         ZombieSpeed { speed: 18. },
//         ZombieBehavior::default(),
//         ZombieAtkTimer::default(),
//         ZombieTargetPlant::default(),
//         ZombieHealth::new(100.0),
//         ZombieDamage { damage: 10.0 },
//         ZombieDefender::normal(),
//         UiTimer::zombie_polevaulting_run(),
//         Transform {
//             translation: zombie_translation,
//             scale: Vec3::splat(2.0),
//             ..default()
//         },
//     ));
// }

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
        if health.is_dead() {
            info!("Zombie despawned at position: {:?}", transform.translation);
            commands.entity(entity).despawn();
            // 发送一个事件，用实体播放僵尸死亡动画
        }
    }
}

pub fn zombie_recover_walk_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_query: Query<(&mut ZombieBehavior, &mut ZombieTargetPlant), With<Zombie>>,
    mut zombie_target_not_exist_reader: EventReader<ZombieTargetNotExistEvent>,
) {
    for event in zombie_target_not_exist_reader.read() {
        if let Ok((mut zombie_behavior, mut zombie_target)) = zombie_query.get_mut(event.zombie) {
            if zombie_behavior.is_attack() {
                *zombie_behavior = ZombieBehavior::Walk;
                zombie_target.clear_target();
                // 切换贴图
                commands.entity(event.zombie).insert(UiTimer::zombie_type0());
                commands.entity(event.zombie).insert(get_zombie_sprite(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    0,
                ));
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

pub fn break_zombie_defender(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_query: Query<(&mut ZombieDefender, &ZombieBehavior), With<Zombie>>,
    mut zombie_defender_broken_reader: EventReader<ZombieDefenderBrokenEvent>,
) {
    for event in zombie_defender_broken_reader.read() {
        if let Ok((mut zombie_defender, zombie_behavior)) = zombie_query.get_mut(event.zombie) {
            zombie_defender.clear_defender();
            let (zombie_sprite, zombie_ui_timer) = 
            if zombie_behavior.is_attack() {
                // 如果是攻击状态，切换到普通僵尸贴图
                (
                    get_zombie_attack_sprite(&asset_server, &mut texture_atlas_layouts),
                    UiTimer::zombie_attack(),
                )
            } else {
                // 如果是行走状态，切换到行走僵尸贴图
                (
                    get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 1),
                    UiTimer::zombie_type1(),
                )
            };
            (
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 0),
                UiTimer::zombie_type0(),
            );

            commands.entity(event.zombie).insert(zombie_sprite);
            commands.entity(event.zombie).insert(zombie_ui_timer);
        } else {
            info!("Zombie {} not found for defender break", event.zombie);
        }
    }
}
