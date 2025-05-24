use bevy::prelude::*;

use crate::model::components::UiTimer;
use crate::model::plant::{Plant};
use crate::model::zombie::{Zombie};

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
