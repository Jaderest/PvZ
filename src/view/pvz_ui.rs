use bevy::{prelude::*, text::cosmic_text::Change};

use crate::model::sun::SunAmount;

#[derive(Component)]
pub struct SunBankUI;

#[derive(Component)]
pub struct SunBankText;


pub struct MyUIPlugin;
impl Plugin for MyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bank_ui)
            .add_systems(Update, update_sun_bank_ui);
    }
}

fn setup_bank_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(60.0),
            height: Val::Percent(20.0),

            grid_template_columns: vec![GridTrack::min_content(); 5],
            grid_template_rows: vec![GridTrack::min_content(); 1],

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    ImageNode {
                        image: asset_server.load("Simple/SunBank.png"),
                        ..default()
                    }
                    .with_mode(NodeImageMode::Auto),
                    Node {
                        display: Display::Grid,

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        height: Val::Percent(100.0),
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
                                Text::new("Sun"),
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
        });
}


fn update_sun_bank_ui(
    sun_amount: Res<SunAmount>,
    mut sun_bank_ui: Query<&mut Text, With<SunBankText>>,
) {
    for mut text in sun_bank_ui.iter_mut() {
        **text = sun_amount.get().to_string();
    }
}