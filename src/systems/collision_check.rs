use bevy::{
    ecs::event,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::model::components::*;
use crate::model::plant::*;
use crate::model::projectile::*;
use crate::model::zombie::*;
use crate::model::{events::*, zombie};

pub fn detect_pea_zombie_collision(
    mut pea_query: Query<(Entity, &ProjDamage, &Transform, &ProjRow, &mut Hit), With<Pea>>,
    mut zombie_query: Query<(Entity, &Transform, &ZombiePosition), With<Zombie>>,
    mut collision_event_writer: EventWriter<PeaHitZombieEvent>,
) {
    for (pea_entity, damage, pea_transform, pea_row, mut hit) in pea_query.iter_mut() {
        for (zombie_entity, zombie_transform, zombie_position) in zombie_query.iter_mut() {
            if hit.is_hit() {
                continue; // Skip if the pea has already hit something
            }
            if zombie_position.y != pea_row.0 {
                continue;
            }
            let mut zombie_center = zombie_transform.translation.truncate();
            zombie_center.x += 50.0;
            let zombie_aabb = Aabb2d::new(zombie_center, Vec2::new(85.0 / 2.0, 129.0 / 2.0));

            let pea_circle = BoundingCircle::new(pea_transform.translation.truncate(), 28.0);

            if pea_collision(pea_circle, zombie_aabb) {
                collision_event_writer.write(PeaHitZombieEvent {
                    pea: pea_entity,
                    zombie: zombie_entity,
                    damage: damage.0,
                });
                hit.set_hit(true);
                break;
            }
        }
    }
}

fn pea_collision(pea: BoundingCircle, zombie: Aabb2d) -> bool {
    pea.intersects(&zombie)
}

pub fn handle_pea_hit_zombie(
    mut commands: Commands,
    mut events_reader: EventReader<PeaHitZombieEvent>,
    mut zombie_query: Query<&mut ZombieHealth, With<Zombie>>,
) {
    for event in events_reader.read() {
        if let Ok(mut zombie_health) = zombie_query.get_mut(event.zombie) {
            zombie_health.current -= event.damage;
            info!("zombie {}, health: {}", event.zombie, zombie_health.current);
        }
        commands.entity(event.pea).despawn();

        // 这里可以添加更多处理，比如播放音效，生成爆炸粒子
    }
}

// 僵尸攻击植物，给僵尸添加一个状态、Walk/Attack，然后根据这个判断僵尸走不走
// 同时发送状态切换到Attack的事件，如果僵尸状态改变，换一次贴图和UiTimer，并且给僵尸添加一个攻击计时器，攻击植物
fn detect_zombie_plant_collision(
    mut plant_query: Query<(Entity, &Transform, &GridPosition), With<Plant>>,
    mut zombie_query: Query<(Entity, &Transform, &ZombiePosition), With<Zombie>>,
) {
}
