use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;

use crate::model::projectile::*;
use crate::model::projectile_events::*;
use crate::view::get_sprites::*;


pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPeaEvent>()
            .add_systems(Update, spawn_pea)
            .add_systems(Update, move_pea)
            .add_systems(Update, despawn_pea)
            ;
    }
}

fn spawn_pea(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_pea_event_reader: EventReader<SpawnPeaEvent>,
) {
    for event in spawn_pea_event_reader.read() {
        let row = event.start_grid.y();
        let mut start_translation = event.start;
        start_translation.y += 50.0; // Adjust the Y position to start above the grid
        start_translation.x += 50.0; // Adjust the X position to start above the grid
        commands.spawn((
            get_pea_sprite(&asset_server),
            Pea,
            Transform {
                translation: start_translation,
                scale: Vec3::splat(2.),
                ..default()
            },
            ProjRow(row),
            Velocity::get_pea(),
            ProjLife::default(),
        ));
    }
}

fn move_pea(
    mut pea_query: Query<(&mut Transform, &Velocity), With<Pea>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in pea_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn despawn_pea(
    mut commands: Commands,
    mut pea_query: Query<(Entity, &mut ProjLife), With<Pea>>,
    time: Res<Time>,
) {
    for (entity, mut life) in pea_query.iter_mut() {
        life.tick(time.delta());
        if life.finished() {
            commands.entity(entity).despawn();
        }
    }
}