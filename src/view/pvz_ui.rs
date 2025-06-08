use bevy::prelude::*;

use crate::config::*;
use crate::model::{plant_events::*, sun::SunAmount, sun_events::SunChangeEvent};

use crate::view::get_sprites::*;

#[derive(Component)]
pub struct SunBankUI;

#[derive(Component)]
pub struct SunBankText;

#[derive(Component)]
pub struct CardUI;

#[derive(Component)]
pub struct ShovelUI;

#[derive(Event, Clone)]
pub struct ClearCardEvent {
    pub target_card: Entity,
    pub plant_type: PlantType,
}

#[derive(Event, Clone)]
pub struct SetCardCDEvent {
    pub plant_type: PlantType,
    pub cd: f32,
}

#[derive(Component)]
pub struct CardCDTimer {
    pub timer: Timer,
    pub max_cd: f32,
}
// 是由放下开始计时
impl CardCDTimer {
    pub fn new(max_cd: f32) -> Self {
        Self {
            timer: Timer::from_seconds(max_cd, TimerMode::Once),
            max_cd,
        }
    }
    pub fn new_not_started(max_cd: f32) -> Self {
        Self {
            timer: Timer::from_seconds(0., TimerMode::Once),
            max_cd,
        }
    }
    pub fn reset(&mut self) {
        self.timer = Timer::from_seconds(self.max_cd, TimerMode::Once);
    }
}
#[derive(Component)]
pub struct CardCDText;

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
                parent
                    .spawn(card(&asset_server, &mut texture_atlas_layouts, i))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("CD".to_string()),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            match i {
                                0 => PlantType::Sunflower,
                                1 => PlantType::PeaShooter,
                                2 => PlantType::WallNut,
                                _ => unreachable!(),
                            },
                            TextColor(Color::srgb(0.1, 0.1, 0.1)),
                            CardCDText,
                        ));
                    });
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

pub fn card(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    index: usize,
) -> impl Bundle + use<> {
    let card_image_node = match index {
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
    let cd_timer = match plant_type {
        PlantType::Sunflower => CardCDTimer::new_not_started(7.5),
        PlantType::PeaShooter => CardCDTimer::new(7.5),
        PlantType::WallNut => CardCDTimer::new(30.0),
    };
    (
        card_image_node,
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
        cd_timer,
        Button,
        CardUI,
    )
}

pub fn card_click_system(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut ImageNode,
            &PlantType,
            &mut CardCDTimer,
        ),
        (Changed<Interaction>, With<Button>, With<CardUI>),
    >,
    mouse: Res<ButtonInput<MouseButton>>,
    mut plant_type: ResMut<PlantType>,
    mut control_state: ResMut<ControlState>,
    mut clear_card_event_writer: EventWriter<ClearCardEvent>,
) {
    for (card_entity, interaction, mut image_node, card_plant_type, timer) in
        interaction_query.iter_mut()
    {
        if !timer.timer.finished() {
            // 如果卡牌还在CD中, 不处理点击事件
            continue;
        }
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
                            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                                if texture_atlas.index == 0 {
                                    clear_card_event_writer.write(ClearCardEvent {
                                        target_card: card_entity,
                                        plant_type: *card_plant_type,
                                    });
                                } else {
                                    texture_atlas.index = 0;
                                    *control_state = ControlState::Normal;
                                    info!("Switch to Normal");
                                }
                            }
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

pub fn clear_card_system(
    mut clear_card_event_reader: EventReader<ClearCardEvent>,
    mut plant_type: ResMut<PlantType>,
    mut card_query: Query<&mut ImageNode, (With<Button>, With<CardUI>)>,
) {
    for event in clear_card_event_reader.read() {
        for mut image_node in card_query.iter_mut() {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 0;
            }
        }
        if let Ok(mut image_node) = card_query.get_mut(event.target_card) {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 1;
                *plant_type = event.plant_type;
            }
        } else {
            info!(
                "Card entity {:?} not found for clear event.",
                event.target_card
            );
        }
    }
}

pub fn card_plant_event(
    mut suc_spawn_plant_reader: EventReader<SuccessSpawnPlantEvent>,
    mut fail_spawn_plant_reader: EventReader<FailedSpawnPlantEvent>,
    mut despawn_plant_reader: EventReader<ShovelPlantEvent>,
    plant_type: Res<PlantType>,
    mut control_state: ResMut<ControlState>,
    mut card_query: Query<
        (&mut ImageNode, &mut CardCDTimer, &PlantType),
        (With<Button>, With<CardUI>),
    >,
) {
    for _event in suc_spawn_plant_reader.read() {
        match *control_state {
            ControlState::SelectPlant => {
                *control_state = ControlState::Normal;
                for (mut image_node, mut timer, card_plant_type) in card_query.iter_mut() {
                    if *plant_type == *card_plant_type {
                        timer.reset();
                    }
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
        for (mut image_node, mut _timer, mut _type) in card_query.iter_mut() {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 0;
            }
        }
    }
    for _event in despawn_plant_reader.read() {
        *control_state = ControlState::Normal;
        for (mut image_node, mut _timer, mut _type) in card_query.iter_mut() {
            if let Some(texture_atlas) = image_node.texture_atlas.as_mut() {
                texture_atlas.index = 0;
            }
        }
    }
}

pub fn card_cd_tick_system(
    time: Res<Time>,
    mut card_query: Query<(&mut CardCDTimer, &PlantType), (With<Button>, With<CardUI>)>,
    mut set_card_cd_event_writer: EventWriter<SetCardCDEvent>,
) {
    for (mut timer, plant_type) in card_query.iter_mut() {
        if timer.timer.tick(time.delta()).finished() {
            set_card_cd_event_writer.write(SetCardCDEvent {
                plant_type: *plant_type,
                cd: 0.0,
            });
        } else {
            set_card_cd_event_writer.write(SetCardCDEvent {
                plant_type: *plant_type,
                cd: timer.max_cd - timer.timer.elapsed_secs(),
            });
        }
    }
}

pub fn card_cd_update_system(
    mut set_card_cd_event_reader: EventReader<SetCardCDEvent>,
    mut card_query: Query<(&mut Text, &PlantType), With<CardCDText>>,
) {
    for event in set_card_cd_event_reader.read() {
        for (mut text, plant_type) in card_query.iter_mut() {
            if *plant_type == event.plant_type {
                **text = format!("{:.1}", event.cd);
                if event.cd <= 0.0 {
                    **text = "".to_string();
                }
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
