use bevy::app::AppExit;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

use crate::asset_loader::{SceneAssets, UiAssets};
use crate::spaceship::Player;
use crate::speedometer::SpeedBarBlackIndicator;
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
        .add_systems(PostStartup, spawn_background_image_system) // runs only once at Startup sequence
        .add_systems(OnEnter(AppState::Setup), spawn_scores_text_system)
        .add_systems(
            OnEnter(GameState::Landed),
            (update_text_score_system, update_text_high_score_system),
        )
        .add_systems(
            Update,
            (
                rotate_background_image_system,
                handle_any_key_has_been_pressed_system.run_if(in_state(GameState::Landed)),
                handle_exit_key_pressed_system.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

// Systems
fn update_text_score_system(mut query: Query<&mut Text, With<TextScore>>, scores: Res<Scores>) {
    let Ok(mut score) = query.get_single_mut() else {
        return;
    };
    score.sections[0].value = scores.score.to_string();
}

fn update_text_high_score_system(
    mut query: Query<&mut Text, With<TextHiScore>>,
    scores: Res<Scores>,
) {
    let Ok(mut score) = query.get_single_mut() else {
        return;
    };
    score.sections[0].value = scores.hi_score.to_string();
}

fn spawn_scores_text_system(mut commands: Commands, assets: ResMut<UiAssets>, scores: Res<Scores>) {
    commands.spawn((
        StateScoped(AppState::Game),
        Resettable,
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
        Resettable,
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
        Resettable,
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
        Resettable,
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

fn spawn_background_image_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
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
    mut spaceship_query: Query<Entity, With<Player>>,
    mut speed_bar_black_indicator_query: Query<Entity, With<SpeedBarBlackIndicator>>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if inputs.just_pressed(KeyCode::Space) {
        info!("space key has been pressed");
        for entity in resettable_text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let Ok(spaceship) = spaceship_query.get_single_mut() else {
            return;
        };
        let Ok(speed_bar_black_indicator) = speed_bar_black_indicator_query.get_single_mut() else {
            return;
        };
        commands.entity(spaceship).despawn_recursive();
        commands
            .entity(speed_bar_black_indicator)
            .despawn_recursive();
        game_state.set(GameState::Setup);
    }
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
