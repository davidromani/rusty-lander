use crate::asset_loader::{MusicAssets, SceneAssets, UiAssets};
use crate::audio::{MusicBeginSoundEffect, MusicPlayingSoundEffect};
use crate::collider::Platform;
use crate::menu::BLACK_COLOR;
use crate::spaceship::{AirScapeSoundEffect, Player, ThrusterSoundEffect};
use crate::state::{AppState, GameState};
use crate::WINDOW_HEIGHT;
use avian2d::prelude::{GravityScale, LinearVelocity};
use bevy::app::AppExit;
use bevy::audio::PlaybackMode;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::text::Text2dBounds;
use bevy_persistent::{Persistent, StorageFormat};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub const FUEL_QUANTITY: f32 = 1000.0;
const INFO_PANEL_WIDTH: f32 = 400.0;
const INFO_PANEL_HEIGHT: f32 = 110.0;
const INFO_PANEL_SIZE: Vec2 = Vec2::new(INFO_PANEL_WIDTH, INFO_PANEL_HEIGHT);
const INFO_PANEL_POSITION: Vec2 = Vec2::new(0.0, WINDOW_HEIGHT / 4.0);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores {
            score: 0,
            hi_score: 0,
            fuel_quantity: FUEL_QUANTITY,
            gravity: 1.0,
        })
        .add_event::<SpaceshipJustLandedEvent>()
        .add_event::<OutOfFuelEvent>()
        .add_systems(Startup, persist_hi_score)
        .add_systems(
            OnEnter(AppState::Menu),
            spawn_rusty_planet_menu_background_image_and_intro_music_system,
        )
        .add_systems(
            OnEnter(AppState::Game),
            (spawn_background_image_system, spawn_scores_text_system),
        )
        .add_systems(
            Update,
            (
                rotate_background_image_system,
                update_scoring_text_system.run_if(in_state(GameState::Landed)),
                catch_spaceship_just_landed_event_system.run_if(in_state(GameState::Landed)),
                catch_out_of_fuel_event_system.run_if(in_state(GameState::Landing)),
                handle_any_control_key_has_been_pressed_system.run_if(in_state(GameState::Landed)),
                handle_exit_key_pressed_system.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

// Systems
fn persist_hi_score(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join("RustyLander");
    commands.insert_resource(
        Persistent::<BestScoreSoFar>::builder()
            .name("scores")
            .format(StorageFormat::Json)
            .path(config_dir.join("scores.json"))
            .default(BestScoreSoFar {
                hi_score: 0,
                gravity: 1.0,
            })
            .build()
            .expect("failed to initialize initial scores"),
    )
}

fn catch_spaceship_just_landed_event_system(
    assets: ResMut<UiAssets>,
    music_assets: Res<MusicAssets>,
    air_scape_sound_controller: Query<&AudioSink, With<AirScapeSoundEffect>>,
    thruster_sound_controller: Query<&AudioSink, With<ThrusterSoundEffect>>,
    music_begin_controller: Query<&AudioSink, With<MusicBeginSoundEffect>>,
    music_playing_controller: Query<&AudioSink, With<MusicPlayingSoundEffect>>,
    mut events_reader: EventReader<SpaceshipJustLandedEvent>,
    mut spaceship_gravity_query: Query<&mut GravityScale, With<Player>>,
    mut commands: Commands,
    mut best_score_so_far: ResMut<Persistent<BestScoreSoFar>>,
    mut scores: ResMut<Scores>,
) {
    if let Ok(sink) = air_scape_sound_controller.get_single() {
        sink.pause();
    }
    if let Ok(sink) = thruster_sound_controller.get_single() {
        sink.pause();
    }
    if let Ok(sink) = music_begin_controller.get_single() {
        sink.pause();
    }
    if let Ok(sink) = music_playing_controller.get_single() {
        sink.pause();
    }
    for event in events_reader.read() {
        let platform = event.platform.clone();
        let linear_velocity = event.linear_velocity;
        let points = (14.57 * linear_velocity.y) as i32 + 720;
        let mut new_score = platform.factor * points;
        scores.score += new_score;
        if best_score_so_far.hi_score < scores.score {
            scores.hi_score = scores.score;
        } else {
            scores.hi_score = best_score_so_far.hi_score;
        }
        commands
            .spawn((
                StateScoped(GameState::Landing),
                Resettable,
                SpriteBundle {
                    sprite: Sprite {
                        color: BLACK_COLOR,
                        custom_size: Some(INFO_PANEL_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(INFO_PANEL_POSITION.extend(11.0)),
                    ..default()
                },
                AudioBundle {
                    source: music_assets.music_end.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        ..default()
                    },
                },
            ))
            .with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        points.to_string()
                            + " x "
                            + platform.factor.to_string().as_str()
                            + " = "
                            + new_score.to_string().as_str(),
                        TextStyle {
                            font: assets.font_vt323.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_justify(JustifyText::Left),
                    text_2d_bounds: Text2dBounds {
                        size: INFO_PANEL_SIZE,
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 20.0, 1.0)),
                    ..default()
                });
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        "press enter key to continue",
                        TextStyle {
                            font: assets.font_vt323.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_justify(JustifyText::Left),
                    text_2d_bounds: Text2dBounds {
                        size: INFO_PANEL_SIZE,
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, -20.0, 1.0)),
                    ..default()
                });
            });
        if new_score > scores.get_available_fuel_quantity() as i32 {
            new_score = scores.get_available_fuel_quantity() as i32;
        }
        scores.fuel_quantity += (new_score as f32) / 5.0;
        let Ok(mut spaceship_gravity) = spaceship_gravity_query.get_single_mut() else {
            return;
        };
        scores.gravity += 0.1;
        spaceship_gravity.0 = scores.gravity;
        if scores.score > best_score_so_far.hi_score {
            best_score_so_far
                .update(|best_score_so_far| {
                    best_score_so_far.hi_score = scores.score;
                })
                .expect("failed to update best_score_so_far gravity");
        }
        if scores.gravity > best_score_so_far.gravity {
            best_score_so_far
                .update(|best_score_so_far| {
                    best_score_so_far.gravity = scores.gravity;
                })
                .expect("failed to update best_score_so_far gravity");
        }
    }
}

fn catch_out_of_fuel_event_system(
    assets: ResMut<UiAssets>,
    mut events_reader: EventReader<OutOfFuelEvent>,
    mut commands: Commands,
) {
    for _event in events_reader.read() {
        commands
            .spawn((
                Resettable,
                SpriteBundle {
                    sprite: Sprite {
                        color: BLACK_COLOR,
                        custom_size: Some(INFO_PANEL_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(INFO_PANEL_POSITION.extend(11.0)),
                    ..default()
                },
            ))
            .with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        "Out of fuel",
                        TextStyle {
                            font: assets.font_vt323.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_justify(JustifyText::Left),
                    text_2d_bounds: Text2dBounds {
                        size: INFO_PANEL_SIZE,
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                });
            });
    }
}

fn update_scoring_text_system(
    scores: Res<Scores>,
    mut score_text_query: Query<&mut Text, (With<TextScore>, Without<TextHiScore>)>,
    mut hi_score_text_query: Query<&mut Text, (With<TextHiScore>, Without<TextScore>)>,
) {
    let Ok(mut score_text) = score_text_query.get_single_mut() else {
        return;
    };
    let Ok(mut hi_score_text) = hi_score_text_query.get_single_mut() else {
        return;
    };
    score_text.sections[0].value = scores.score.to_string();
    hi_score_text.sections[0].value = scores.hi_score.to_string();
}

fn spawn_scores_text_system(
    mut commands: Commands,
    assets: ResMut<UiAssets>,
    scores: Res<Scores>,
    best_score_so_far: Res<Persistent<BestScoreSoFar>>,
) {
    // black background UI horizontal
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -330.0, 2.0)),
            sprite: Sprite {
                color: BLACK_COLOR,
                custom_size: Some(Vec2::new(1024.0, 60.0)),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(2),
    ));
    // black background UI vertical
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(485.0, 0.0, 2.0)),
            sprite: Sprite {
                color: BLACK_COLOR,
                custom_size: Some(Vec2::new(50.0, 720.0)),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(2),
    ));
    // speedometer UI texts
    commands.spawn((
        StateScoped(AppState::Game),
        TextBundle::from_section(
            "m/s",
            TextStyle {
                font: assets.font_vt323.clone(),
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(36.0),
            right: Val::Px(16.0),
            ..default()
        }),
    ));
    // scoring UI texts
    commands.spawn((
        StateScoped(AppState::Game),
        TextBundle::from_section(
            "Score",
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(33.0),
            left: Val::Px(20.0),
            ..default()
        }),
    ));
    commands.spawn((
        StateScoped(AppState::Game),
        TextScore,
        TextBundle::from_section(
            scores.score.to_string(),
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(33.0),
            left: Val::Px(120.0),
            ..default()
        }),
    ));
    commands.spawn((
        StateScoped(AppState::Game),
        TextBundle::from_section(
            "High Score",
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(33.0),
            left: Val::Px(400.0),
            ..default()
        }),
    ));
    commands.spawn((
        StateScoped(AppState::Game),
        TextHiScore,
        TextBundle::from_section(
            best_score_so_far.hi_score.to_string(),
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(33.0),
            left: Val::Px(538.0),
            ..default()
        }),
    ));
}

fn spawn_rusty_planet_menu_background_image_and_intro_music_system(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    music_assets: Res<MusicAssets>,
) {
    commands.spawn((
        StateScoped(AppState::Menu),
        SpriteBundle {
            texture: scene_assets.rusty_planet.clone(),
            transform: Transform {
                scale: Vec3::new(0.85, 0.84, 1.0),
                ..default()
            },
            ..default()
        },
        AudioBundle {
            source: music_assets.music_intro.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
    ));
}

fn spawn_background_image_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            texture: scene_assets.background.clone(),
            transform: Transform {
                rotation: Quat::from_rotation_z(thread_rng().gen_range(0.0..1.0)),
                ..default()
            },
            ..default()
        },
        Background,
        Rotatable { speed: -0.001 },
    ));
}

fn rotate_background_image_system(
    mut query: Query<(&mut Transform, &Rotatable), With<Background>>,
    timer: Res<Time>,
) {
    let Ok((mut transform, background)) = query.get_single_mut() else {
        return;
    };
    // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
    // and then multiplied by delta_seconds which is the time that passed last frame.
    // In other words. Speed is equal to the amount of rotations per second.
    transform.rotate_z(background.speed * TAU * timer.delta_seconds());
}

fn handle_exit_key_pressed_system(mut exit: EventWriter<AppExit>) {
    info!("exit key has been pressed");
    exit.send(AppExit::Success);
}

fn handle_any_control_key_has_been_pressed_system(
    inputs: Res<ButtonInput<KeyCode>>,
    resettable_text_query: Query<Entity, With<Resettable>>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if inputs.just_pressed(KeyCode::ControlLeft)
        || inputs.just_pressed(KeyCode::ControlRight)
        || inputs.just_pressed(KeyCode::Enter)
    {
        for entity in resettable_text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game_state.set(GameState::Setup);
    }
}

// Sets
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameSet {
    Physics,
    Collisions,
    SpeedBar,
}

// Events
#[derive(Event)]
pub struct SpaceshipJustLandedEvent {
    pub platform: Platform,
    pub linear_velocity: LinearVelocity,
}

#[derive(Event)]
pub struct OutOfFuelEvent;

// Components
#[derive(Component)]
struct Background;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

#[derive(Component)]
struct TextScore;

#[derive(Component)]
struct TextHiScore;

#[derive(Component)]
pub struct Resettable;

// Resources (global scope allocated data)
#[derive(Resource)]
pub struct WorldBoundsVertices2D {
    pub data: Vec<Vec2>,
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct BestScoreSoFar {
    pub hi_score: i32,
    pub gravity: f32,
}

#[derive(Resource, Debug)]
pub struct Scores {
    pub score: i32,
    pub hi_score: i32,
    pub fuel_quantity: f32,
    pub gravity: f32,
}

impl Scores {
    pub fn get_available_fuel_quantity(&self) -> f32 {
        FUEL_QUANTITY - self.fuel_quantity
    }
}

#[test]
fn dummy_test() {
    let test = 1;
    assert_eq!(test, 1);
}
