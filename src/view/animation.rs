use bevy::prelude::*;
use bevy::{
    animation::{AnimationTarget, AnimationTargetId, animated_field},
    prelude::*,
};

pub struct AnimationInfo {
    pub target_name: Name,
    pub target_id: AnimationTargetId,
    pub graph: Handle<AnimationGraph>,
    pub node_index: AnimationNodeIndex,
}
impl AnimationInfo {
    pub fn create_sunflower(
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

    pub fn create_sun(
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
    
    pub fn create_pea(
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
        start: Vec3,
        end: Vec3,
    ) -> AnimationInfo {
        let animation_target_name = Name::new("Pea");
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

        let mut animation_clip = AnimationClip::default();

        let animation_domain = interval(0.0, 1.0).unwrap();

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
