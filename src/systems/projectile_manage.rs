use bevy::{prelude::*, state::commands};

use crate::model::projectile::*;
use crate::model::projectile_events::*;
use crate::config::*;
use crate::view::animation::AnimationInfo;


fn spawn_pea( // 需要添加动画好像
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut spawn_pea_event_reader: EventReader<SpawnPeaEvent>,

    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    for event in spawn_pea_event_reader.read() {
        let start_grid = event.start_grid;
        let start_position = grid2pixel(*game_config, start_grid.x() as f32, start_grid.y() as f32, 4.);
        // 有个不太好的地方是我豌豆飞行速度是由这个常数1500.决定的
        // 我真的要用这个动画吗？还是手动控制velocity
        let end_position = Vec3::new(
            start_position.x + 1500.,
            start_position.y,
            start_position.z,
        );
        let AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph,
            node_index: animation_node_index,
        } = AnimationInfo::create_pea(
            &mut animation_graphs,
            &mut animation_clips,
            start_position,
            end_position,
        );
    }
}