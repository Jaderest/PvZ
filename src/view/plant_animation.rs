use bevy::prelude::*;

use crate::model::plant::{Plant, UiTimer};

pub fn play_plant_animation(
    mut plant_query: Query<(&mut Sprite, &mut UiTimer), With<Plant>>,
    time: Res<Time>,
) {
    for (mut sprite, mut timer) in plant_query.iter_mut() {
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            if timer.timer.tick(time.delta()).just_finished() {
                timer.index += 1;
                if timer.index > timer.max_index {
                    timer.index = 0;
                }
                texture_atlas.index = timer.index;
            }
        }
    }
}
