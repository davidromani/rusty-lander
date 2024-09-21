use avian2d::prelude::LinearVelocity;
use bevy::app::AppExit;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

use crate::asset_loader::{SceneAssets, UiAssets};
use crate::collider::Platform;
use crate::menu::BLACK_COLOR;
use crate::state::{AppState, GameState};

pub const FUEL_QUANTITY: f32 = 1000.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores {
            score: 0,
            hi_score: 0,
            fuel_quantity: FUEL_QUANTITY,
        })
        .add_event::<SpaceshipJustLandedEvent>()
        .add_systems(
            OnEnter(AppState::Menu),
            spawn_rusty_planet_menu_background_image_system,
        )
        .add_systems(
            OnEnter(AppState::Game),
            (spawn_background_image_system, spawn_scores_text_system),
        )
        .add_systems(OnEnter(GameState::Landed), update_scoring_text_system)
        .add_systems(
            Update,
            (
                rotate_background_image_system,
                catch_spaceship_just_landed_event_system.run_if(in_state(GameState::Landed)),
                handle_any_key_has_been_pressed_system.run_if(in_state(GameState::Landed)),
                handle_exit_key_pressed_system.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

// Systems
fn catch_spaceship_just_landed_event_system(
    assets: ResMut<UiAssets>,
    mut events_reader: EventReader<SpaceshipJustLandedEvent>,
    mut commands: Commands,
    mut scores: ResMut<Scores>,
) {
    for event in events_reader.read() {
        let platform = event.platform.clone();
        let linear_velocity = event.linear_velocity.clone();
        let points = (14.57 * linear_velocity.y) as i16 + 720;
        let mut new_score = platform.factor * points;
        scores.score += new_score;
        if scores.hi_score < scores.score {
            scores.hi_score = scores.score;
        }
        commands.spawn((
            Resettable,
            TextScoringAfterLanding,
            TextBundle::from_section(
                points.to_string()
                    + " x "
                    + platform.factor.to_string().as_str()
                    + " = "
                    + new_score.to_string().as_str(),
                TextStyle {
                    font: assets.font_vt323.clone(),
                    font_size: 60.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(30.0),
                left: Val::Px(88.0),
                ..default()
            }),
        ));
        commands.spawn((
            Resettable,
            TextScoringAfterLanding,
            TextBundle::from_section(
                "press space bar key to continue",
                TextStyle {
                    font: assets.font_vt323.clone(),
                    font_size: 30.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(80.0),
                left: Val::Px(88.0),
                ..default()
            }),
        ));
        if new_score > scores.get_available_fuel_quantity() as i16 {
            new_score = scores.get_available_fuel_quantity() as i16;
        }
        scores.fuel_quantity += new_score as f32;
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

fn spawn_scores_text_system(mut commands: Commands, assets: ResMut<UiAssets>, scores: Res<Scores>) {
    // black background UI horizontal
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -330.0, 2.0)),
            sprite: Sprite {
                color: BLACK_COLOR,
                custom_size: Some(Vec2::new(1280.0, 60.0)),
                ..default()
            },
            ..default()
        },
    ));
    // black background UI vertical
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(620.0, 0.0, 2.0)),
            sprite: Sprite {
                color: BLACK_COLOR,
                custom_size: Some(Vec2::new(40.0, 720.0)),
                ..default()
            },
            ..default()
        },
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
            bottom: Val::Px(30.0),
            left: Val::Px(88.0),
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
            bottom: Val::Px(30.0),
            left: Val::Px(188.0),
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
            bottom: Val::Px(30.0),
            left: Val::Px(500.0),
            ..default()
        }),
    ));
    commands.spawn((
        StateScoped(AppState::Game),
        TextHiScore,
        TextBundle::from_section(
            scores.hi_score.to_string(),
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(638.0),
            ..default()
        }),
    ));
}

fn spawn_rusty_planet_menu_background_image_system(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        StateScoped(AppState::Menu),
        SpriteBundle {
            texture: scene_assets.rusty_planet.clone(),
            transform: Transform {
                scale: Vec3::new(1.07, 1.07, 1.0),
                ..default()
            },
            ..default()
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

fn handle_any_key_has_been_pressed_system(
    inputs: Res<ButtonInput<KeyCode>>,
    resettable_text_query: Query<Entity, With<Resettable>>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if inputs.just_pressed(KeyCode::Space) {
        info!("space key has been pressed");
        for entity in resettable_text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game_state.set(GameState::Setup);
    }
}

// Events
#[derive(Event)]
pub struct SpaceshipJustLandedEvent {
    pub platform: Platform,
    pub linear_velocity: LinearVelocity,
}

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
pub struct TextScoringAfterLanding;

#[derive(Component)]
pub struct Resettable;

// Resources (global scope allocated data)
#[derive(Resource, Debug)]
pub struct Scores {
    pub score: i16,
    pub hi_score: i16,
    pub fuel_quantity: f32,
}

impl Scores {
    pub fn get_available_fuel_quantity(&self) -> f32 {
        FUEL_QUANTITY - self.fuel_quantity
    }
}
