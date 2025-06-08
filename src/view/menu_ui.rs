use bevy::prelude::*;
use bevy::color::palettes::css::CRIMSON;

use crate::{model::components::GameState, view::get_sprites::get_menu_imagenode};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct OnMenuScreen;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(Update, (menu_button_interaction, button_system).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
    ;
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up the main menu UI");
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
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    let right_icon = asset_server.load("other/right.png");
    let exit_icon = asset_server.load("other/exit.png");

    info!("finished loading assets for the main menu");
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        get_menu_imagenode(&asset_server),
        OnMenuScreen,
        children![(
            Node {
                margin: UiRect {
                    top: Val::Percent(30.0),
                    left: Val::Percent(30.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // - new game
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("Start"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ],
        )],
    ));
}

fn menu_button_interaction(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>, // bevy提供的
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Quit => {
                    // Exit the application
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    // Change the game state to start the game
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}

fn button_system(
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

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}