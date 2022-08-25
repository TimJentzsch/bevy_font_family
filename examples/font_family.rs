use bevy::prelude::*;
use bevy_font_family::font_weights::*;
use bevy_font_family::prelude::*;

struct FiraSans;

// TODO: Write a derive macro for this
impl FontFamily for FiraSans {
    fn roman_fonts() -> Vec<FontDefinition> {
        vec![
            FontDefinition::new("fonts/fira_sans/FiraSans-Thin.ttf", THIN),
            FontDefinition::new("fonts/fira_sans/FiraSans-Regular.ttf", REGULAR),
            FontDefinition::new("fonts/fira_sans/FiraSans-Bold.ttf", BOLD),
        ]
    }

    fn italic_fonts() -> Vec<FontDefinition> {
        vec![
            FontDefinition::new("fonts/fira_sans/FiraSans-ThinItalic.ttf", THIN),
            FontDefinition::new("fonts/fira_sans/FiraSans-Italic.ttf", REGULAR),
            FontDefinition::new("fonts/fira_sans/FiraSans-Bold.ttf", BOLD),
        ]
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// Spawn the camera and text nodes.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Roman Thin",
                    TextStyle {
                        font: asset_server.load(FiraSans::font().thin()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Italic Thin",
                    TextStyle {
                        font: asset_server.load(FiraSans::font().thin().italic()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Roman Regular",
                    TextStyle {
                        font: asset_server.load(FiraSans::font()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Italic Regular",
                    TextStyle {
                        font: asset_server.load(FiraSans::font().italic()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Roman Bold",
                    TextStyle {
                        font: asset_server.load(FiraSans::font().bold()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Italic Bold",
                    TextStyle {
                        font: asset_server.load(FiraSans::font().bold().italic()),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}
