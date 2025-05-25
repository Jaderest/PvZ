use bevy::input::mouse;
use bevy::prelude::*;

use crate::config::*;
use crate::model::{
    plant_events::*,
    sun::{self, SunAmount},
    sun_events::SunChangeEvent,
};
use crate::view::*;

use super::get_sprites::*;

#[derive(Component)]
pub struct SunBankUI;

#[derive(Component)]
pub struct SunBankText;

#[derive(Component)]
pub struct CardUI;

#[derive(Component)]
pub struct ShovelUI;

pub struct MyUIPlugin;
impl Plugin for MyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bank_ui)
            .add_systems(Update, update_sun_bank_ui)
            .add_systems(Update, card_click_system)
            .add_systems(Update, card_plant_event)
            .add_systems(Update, shovel_click_system)
            .add_systems(Update, shovel_plant_event);
    }
}

pub fn setup_bank_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sun_amount: Res<SunAmount>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(9.0),
            height: Val::Percent(75.0),

            grid_template_columns: vec![GridTrack::min_content(); 1],
            grid_template_rows: vec![GridTrack::min_content(); 5],

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    get_sunbank_imagenode(&asset_server),
                    Node {
                        display: Display::Grid,

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        // height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(0.896552),

                        grid_template_columns: vec![GridTrack::min_content(); 1],
                        grid_template_rows: vec![GridTrack::min_content(); 3],

                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Node {
                        height: Val::Percent(73.0),
                        ..default()
                    });
                    parent
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::min_content(); 1],
                            grid_template_rows: vec![GridTrack::min_content(); 1],
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(sun_amount.get().to_string()),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.1, 0.1, 0.1)),
                                SunBankText,
                            ));
                        });
                    parent.spawn(Node {
                        height: Val::Percent(10.0),
                        ..default()
                    });
                });
            // 生成三个ui卡牌
            for i in 0..3 {
                parent.spawn(card(&asset_server, &mut texture_atlas_layouts, i));
            }
            parent
                .spawn((
                    get_shovel_bank_imagenode(&asset_server),
                    Node {
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::min_content(); 1],
                        grid_template_rows: vec![GridTrack::min_content(); 1],
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        get_shovel_imagenode(&asset_server),
                        Node {
                            width: Val::Percent(100.0),
                            aspect_ratio: Some(1.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ShovelUI,
                        Button,
                        Visibility::Visible,
                    ));
                });
        });
}

pub fn update_sun_bank_ui(
    sun_bank_ui: Single<&mut Text, With<SunBankText>>,
    mut sun_change_event_reader: EventReader<SunChangeEvent>,
) {
    let mut text = sun_bank_ui.into_inner();
    for event in sun_change_event_reader.read() {
        **text = event.0.to_string();
    }
}

// TODO: 为card添加蒙版，来动态显示seed的冷却时间
// TODO: 添加鼠标跟随
pub fn card(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    index: usize,
) -> impl Bundle + use<> {
    let image_node = match index {
        0 => get_sunflower_card_imagenode(asset_server, texture_atlas_layouts),
        1 => get_peashooter_card_imagenode(asset_server, texture_atlas_layouts),
        2 => get_wallnut_card_imagenode(asset_server, texture_atlas_layouts),
        _ => unreachable!(),
    };
    let plant_type = match index {
        0 => PlantType::Sunflower,
        1 => PlantType::PeaShooter,
        2 => PlantType::WallNut,
        _ => unreachable!(),
    };
    (
        image_node,
        Node {
            width: Val::Percent(100.0),
            aspect_ratio: Some(1.666667),
            padding: UiRect::all(Val::Px(5.0)),
            display: Display::Grid,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        plant_type,
        Button,
        CardUI,
    )
}

pub fn card_click_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &PlantType),
        (Changed<Interaction>, With<Button>, With<CardUI>),
    >,
    mouse: Res<ButtonInput<MouseButton>>,
    mut plant_type: ResMut<PlantType>,
    mut control_state: ResMut<ControlState>,
) {
    for (interaction, mut image_node, card_plant_type) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse.just_pressed(MouseButton::Left) {
                    // 处理control_state
                    match *control_state {
                        ControlState::Normal => {
                            *control_state = ControlState::SelectPlant;
                            *plant_type = *card_plant_type;
                            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                                texture_atlas.index = 1;
                            }
                            info!("Switch to SelectPlant");
                        }
                        ControlState::SelectPlant => {
                            *control_state = ControlState::Normal;
                            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                                texture_atlas.index = 0;
                            }
                            info!("Switch to Normal");
                        }
                        ControlState::Shovel => {
                            *control_state = ControlState::Normal;
                            info!("Switch to Normal");
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn card_plant_event(
    mut suc_spawn_plant_reader: EventReader<SuccessSpawnPlantEvent>,
    mut fail_spawn_plant_reader: EventReader<FailedSpawnPlantEvent>,
    mut despawn_plant_reader: EventReader<ShovelPlantEvent>,
    mut control_state: ResMut<ControlState>,
    mut card_query: Query<&mut ImageNode, (With<Button>, With<CardUI>)>,
    // todo: 不知道要不要重构一下PlantType的enum，重置为None
) {
    for _event in suc_spawn_plant_reader.read() {
        match *control_state {
            ControlState::SelectPlant => {
                *control_state = ControlState::Normal;
                for mut image_node in card_query.iter_mut() {
                    if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                        texture_atlas.index = 0;
                    }
                }
            }
            _ => {}
        }
    }
    for _event in fail_spawn_plant_reader.read() {
        *control_state = ControlState::Normal;
        for mut image_node in card_query.iter_mut() {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 0;
            }
        }
    }
    for _event in despawn_plant_reader.read() {
        *control_state = ControlState::Normal;
        for mut image_node in card_query.iter_mut() {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 0;
            }
        }
    }
}

pub fn shovel_click_system(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility),
        (Changed<Interaction>, With<Button>, With<ShovelUI>),
    >,
    mouse: Res<ButtonInput<MouseButton>>,
    mut control_state: ResMut<ControlState>,
) {
    for (interaction, mut visibility) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse.just_pressed(MouseButton::Left) {
                    // 处理control_state
                    match *control_state {
                        ControlState::Normal => {
                            *control_state = ControlState::Shovel;
                            *visibility = Visibility::Hidden;
                            info!("Switch to Shovel");
                        }
                        ControlState::Shovel => {
                            *control_state = ControlState::Normal;
                            *visibility = Visibility::Visible;
                            info!("Switch to Normal");
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn shovel_plant_event(
    mut despawn_plant_reader: EventReader<ShovelPlantEvent>,
    mut interaction_query: Query<&mut Visibility, (With<Button>, With<ShovelUI>)>,
) {
    for _event in despawn_plant_reader.read() {
        for mut visibility in interaction_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
