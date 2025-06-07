use bevy::prelude::*;

use crate::{model::{components::GameState, events::{GameLoseEvent, GameWinEvent}}, view::get_sprites::*};

pub fn result_plugin(app: &mut App) {
    app.add_event::<GameLoseEvent>()
        .add_event::<GameWinEvent>()
        .add_systems(OnEnter(GameState::Success), win_ui_setup)
        .add_systems(OnEnter(GameState::GameOver), lose_ui_setup);
}

pub fn win_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprites
    // let exit_icon = asset_server.load("sprites/exit.png");

    // Create the UI for the win screen
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        get_game_win_imagenode(&asset_server),
    ));
}

pub fn lose_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprites
    // let exit_icon = asset_server.load("sprites/exit.png");

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
    ));
}
