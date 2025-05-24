use bevy::prelude::*;
use rand::Rng;

use crate::config::*;
use crate::model::components::UiTimer;
use crate::model::zombie::*;
use crate::model::zombie_events::*;
use crate::view::get_sprites::get_zombie_sprite;
use crate::view::plant_animation::*;
use crate::systems::keyboard_control::*;

pub struct ZombiePlugin;
impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ZombieSpawnEvent>()
            .add_systems(Update, spawn_zombie)
            .add_systems(Update, play_zombie_animation)
            .add_systems(Update, keyboard_spawn_zombie)
            ;
    }
}

fn spawn_zombie(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut zombie_spawn_event: EventReader<ZombieSpawnEvent>,
) {
    for event in zombie_spawn_event.read() {
        info!("Spawn zombie at y: {}", event.y);
        let zombie_position = ZombiePosition::new(5.0, event.y);
        let mut zombie_translation = grid2pixel(
            *game_config,
            zombie_position.x,
            zombie_position.y as f32,
            7. - event.y as f32 * 0.1,
        );
        //TODO: 检查魔法数字
        zombie_translation.y += 40.0;
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            commands.spawn((
                get_zombie_sprite(&asset_server, &mut texture_atlas_layouts, 0),
                Zombie,
                zombie_position,
                ZombieSpeed { speed: 100. },
                ZombieAtkTimer::default(),
                ZombieHealth {
                    current: 100.0,
                    max: 100.0,
                },
                ZombieDamage { damage: 10.0 },
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
                ZombieSpeed { speed: 100. },
                ZombieAtkTimer::default(),
                ZombieHealth {
                    current: 100.0,
                    max: 100.0,
                },
                ZombieDamage { damage: 10.0 },
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
