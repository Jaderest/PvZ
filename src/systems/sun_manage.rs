use bevy::{
    animation::{AnimationTarget, AnimationTargetId, animated_field},
    prelude::*,
};
use rand::Rng;

use crate::model::sun::*;
use crate::model::sun_events::*;
use crate::{config::*, model::plant_events::SuccessSpawnPlantEvent};

// todo: 为天上生成阳光实现动画

pub struct SunPlugin;
impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        // todo：注册生成阳光
        app.insert_resource(GlobalSunTimer::default())
            .insert_resource(SunAmount::default())
            .add_event::<PickupSunEvent>()
            .add_event::<SpawnFlowerSunEvent>()
            .add_event::<SunChangeEvent>()
            .add_systems(Update, sun_produce_sun)
            .add_systems(Update, sun_add)
            .add_systems(Update, sun_consume)
            .add_systems(Update, sun_despawn_with_time)
            .add_systems(Update, flower_produce_sun);
    }
}

/// 自然生成阳光
fn sun_produce_sun(
    // mut sun_amount: ResMut<SunAmount>,
    time: Res<Time>,
    game_config: Res<GameConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<GlobalSunTimer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let mut rng = rand::rng();
    timer.tick(time.delta());
    if timer.just_finished() {
        let x: f32 = rng.random_range(0.0..8.0);
        let y: f32 = rng.random_range(0.0..4.0);
        
        let sun_position = grid2pixel(*game_config, x, y, 10.0);
        let start_position = Vec3::new(sun_position.x, 500., sun_position.z);

        let AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph,
            node_index: animation_node_index,
        } = AnimationInfo::create_sun(
            &mut animation_graphs,
            &mut animation_clips,
            start_position,
            sun_position,
        );

        let mut animation_player = AnimationPlayer::default();
        animation_player.play(animation_node_index);

        let sun_id = commands
            .spawn((
                Sprite {
                    image: asset_server.load("other/Sun.png"),
                    ..default()
                },
                Sun(25),
                SunPosition { x, y },
                SunDespawnTimer::default(),
                Transform {
                    translation: start_position,
                    scale: Vec3::splat(2.),
                    ..default()
                },
                animation_target_name,
                animation_player,
                AnimationGraphHandle(animation_graph),
            ))
            .id();
        commands.entity(sun_id).insert(AnimationTarget {
            id: animation_target_id,
            player: sun_id,
        });
    }
}

fn flower_produce_sun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut sunflower_produce_reader: EventReader<SpawnFlowerSunEvent>,
) {
    for event in sunflower_produce_reader.read() {
        let start = event.start;
        let end = event.end;
        let amount = event.amount;

        let AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph,
            node_index: animation_node_index,
        } = AnimationInfo::create_sunflower(
            &mut animation_graphs,
            &mut animation_clips,
            start,
            end,
        );

        let mut animation_player = AnimationPlayer::default();
        animation_player.play(animation_node_index);

        let sun_entity = commands
            .spawn((
                Sprite {
                    image: asset_server.load("other/Sun.png"),
                    ..default()
                },
                Sun(amount),
                SunPosition {
                    x: event.sun_position.x,
                    y: event.sun_position.y,
                },
                SunDespawnTimer::default(),
                Transform {
                    translation: start,
                    scale: Vec3::splat(2.),
                    ..default()
                },
                animation_target_name,
                animation_player,
                AnimationGraphHandle(animation_graph),
            ))
            .id();
        commands.entity(sun_entity).insert(AnimationTarget {
            id: animation_target_id,
            player: sun_entity,
        });
    }
}

fn sun_add(
    mut sun_amount: ResMut<SunAmount>,
    mut pickup_sun_reader: EventReader<PickupSunEvent>,
    mut sun_change_writer: EventWriter<SunChangeEvent>,
) {
    for event in pickup_sun_reader.read() {
        sun_amount.add(event.amount);
        sun_change_writer.write(SunChangeEvent(sun_amount.get()));
    }
}

fn sun_consume(
    mut sun_amount: ResMut<SunAmount>,
    mut suc_spawn_plant_reader: EventReader<SuccessSpawnPlantEvent>,
    mut sun_change_writer: EventWriter<SunChangeEvent>,
) {
    for event in suc_spawn_plant_reader.read() {
        sun_amount.sub(event.sun_cost);
        sun_change_writer.write(SunChangeEvent(sun_amount.get()));
    }
}

struct AnimationInfo {
    target_name: Name,
    target_id: AnimationTargetId,
    graph: Handle<AnimationGraph>,
    node_index: AnimationNodeIndex,
}
impl AnimationInfo {
    fn create_sunflower(
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
        start: Vec3,
        end: Vec3,
    ) -> AnimationInfo {
        let animation_target_name = Name::new("Sun");
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

        let mut animation_clip = AnimationClip::default();

        let animation_domain = interval(0.0, 1.0).unwrap();

        let translation_curve = EasingCurve::new(start, end, EaseFunction::CircularInOut)
            .reparametrize_linear(animation_domain)
            .expect("this curve has bounded domain, so this should never fail");
        animation_clip.add_curve_to_target(
            animation_target_id,
            AnimatableCurve::new(animated_field!(Transform::translation), translation_curve),
        );

        let animation_clip_handle = animation_clips.add(animation_clip);

        let (animation_graph, animation_node_index) =
            AnimationGraph::from_clip(animation_clip_handle);
        let animation_graph_handle = animation_graphs.add(animation_graph);

        AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph_handle,
            node_index: animation_node_index,
        }
    }

    fn create_sun(
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
        start: Vec3,
        end: Vec3,
    ) -> AnimationInfo {
        let animation_target_name = Name::new("Sun");
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

        let mut animation_clip = AnimationClip::default();

        let animation_domain = interval(0.0, 5.).unwrap();

        let translation_curve = EasingCurve::new(start, end, EaseFunction::Linear)
            .reparametrize_linear(animation_domain)
            .expect("this curve has bounded domain, so this should never fail");
        animation_clip.add_curve_to_target(
            animation_target_id,
            AnimatableCurve::new(animated_field!(Transform::translation), translation_curve),
        );

        let animation_clip_handle = animation_clips.add(animation_clip);

        let (animation_graph, animation_node_index) =
            AnimationGraph::from_clip(animation_clip_handle);
        let animation_graph_handle = animation_graphs.add(animation_graph);

        AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph_handle,
            node_index: animation_node_index,
        }
    }
}

fn sun_despawn_with_time(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SunDespawnTimer), With<Sun>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
