use bevy::prelude::*;

use crate::model::zombie_events::*;

// 仅仅用于测试，在某行生成僵尸
pub fn keyboard_spawn_zombie(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut zombie_spawn_event_writer: EventWriter<ZombieSpawnEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        zombie_spawn_event_writer.write(ZombieSpawnEvent { y: 0 });
    }
    if keyboard_input.just_pressed(KeyCode::Numpad2) {
        zombie_spawn_event_writer.write(ZombieSpawnEvent { y: 1 });
    }
    if keyboard_input.just_pressed(KeyCode::Numpad3) {
        zombie_spawn_event_writer.write(ZombieSpawnEvent { y: 2 });
    }
    if keyboard_input.just_pressed(KeyCode::Numpad4) {
        zombie_spawn_event_writer.write(ZombieSpawnEvent { y: 3 });
    }
    if keyboard_input.just_pressed(KeyCode::Numpad5) {
        zombie_spawn_event_writer.write(ZombieSpawnEvent { y: 4 });
    }
}