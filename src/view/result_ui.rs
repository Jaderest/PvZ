use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

use crate::{
    model::{
        components::GameState,
        events::{GameLoseEvent, GameWinEvent},
    },
    view::get_sprites::*,
};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

pub fn result_plugin(app: &mut App) {
    app.add_event::<GameLoseEvent>()
        .add_event::<GameWinEvent>()
        .add_systems(OnEnter(GameState::Success), win_ui_setup)
        .add_systems(OnEnter(GameState::GameOver), lose_ui_setup)
        .add_systems(
            Update,
            (set_win_system, set_lose_system).run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, result_button_system.run_if(in_state(GameState::Success)))
        .add_systems(Update, result_button_system.run_if(in_state(GameState::GameOver)))
        .add_systems(Update, result_button_interaction.run_if(in_state(GameState::Success)))
        .add_systems(Update, result_button_interaction.run_if(in_state(GameState::GameOver)));
}

pub fn win_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprites
    let exit_icon = asset_server.load("other/exit.png");

    // Create the UI for the win screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        get_game_win_imagenode(&asset_server),
        children![(
            Node {
                margin: UiRect {
                    top: Val::Percent(30.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // - quit
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Exit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ],
        )],
    ));
}

pub fn lose_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprites
    let exit_icon = asset_server.load("other/exit.png");

    // Create the UI for the win screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    // Create the UI for the lose screen
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        get_game_lose_imagenode(&asset_server), // Placeholder for lose image
        children![(
            Node {
                margin: UiRect {
                    top: Val::Percent(30.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // - quit
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Exit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ],
        )],
    ));
}

pub fn set_win_system(
    mut win_state: ResMut<NextState<GameState>>,
    mut game_win_event_reader: EventReader<GameWinEvent>,
) {
    for _event in game_win_event_reader.read() {
        win_state.set(GameState::Success);
    }
}

pub fn set_lose_system(
    mut lose_state: ResMut<NextState<GameState>>,
    mut game_lose_event_reader: EventReader<GameLoseEvent>,
) {
    for _event in game_lose_event_reader.read() {
        lose_state.set(GameState::GameOver);
    }
}

fn result_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match interaction {
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
            _ => NORMAL_BUTTON.into(),
        }
    }
}

fn result_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>, // bevy提供的
) {
    for interaction in interaction_query.iter() {
        // Exit the application
        if *interaction == Interaction::Pressed {
            app_exit_events.write(AppExit::Success);
            return;
        }
    }
}
