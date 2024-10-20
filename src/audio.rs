use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::asset_loader::AudioAssets;
use crate::game::Scores;
use crate::spaceship::{AirScapeSoundEffect, PlayerAction, ThrusterSoundEffect};
use crate::state::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_audio_entities_system)
            .add_systems(
                Update,
                (
                    play_air_scape_sound_effect_system,
                    play_thruster_sound_effect_system,
                )
                    .run_if(in_state(GameState::Landing)),
            )
            .add_systems(OnEnter(GameState::Crashed), pause_all_sound_effect_system);
    }
}

// Systems
fn spawn_audio_entities_system(audio_assets: Res<AudioAssets>, mut commands: Commands) {
    commands.spawn((
        AirScapeSoundEffect,
        AudioBundle {
            source: audio_assets.ship_air_scape.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
    ));
    commands.spawn((
        ThrusterSoundEffect,
        AudioBundle {
            source: audio_assets.ship_thruster.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
    ));
}

fn play_air_scape_sound_effect_system(
    scores: ResMut<Scores>,
    sound_controller: Query<&AudioSink, With<AirScapeSoundEffect>>,
    mut controllers: Query<&ActionState<PlayerAction>>,
) {
    for action_state in &mut controllers {
        if scores.fuel_quantity > 0.0 {
            if action_state.just_pressed(&PlayerAction::LeftThruster)
                || action_state.just_pressed(&PlayerAction::RightThruster)
            {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.play();
                }
            }
            if action_state.just_released(&PlayerAction::LeftThruster)
                || action_state.just_released(&PlayerAction::RightThruster)
            {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.pause();
                }
            }
        } else {
            if let Ok(sink) = sound_controller.get_single() {
                sink.pause();
            }
        }
    }
}

fn play_thruster_sound_effect_system(
    scores: ResMut<Scores>,
    sound_controller: Query<&AudioSink, With<ThrusterSoundEffect>>,
    mut controllers: Query<&ActionState<PlayerAction>>,
) {
    for action_state in &mut controllers {
        if scores.fuel_quantity > 0.0 {
            if action_state.just_pressed(&PlayerAction::MainThrusterBig) {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.set_volume(1.0);
                    sink.play();
                }
            }
            if action_state.just_pressed(&PlayerAction::MainThrusterMedium) {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.set_volume(0.66);
                    sink.play();
                }
            }
            if action_state.just_pressed(&PlayerAction::MainThrusterSmall) {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.set_volume(0.33);
                    sink.play();
                }
            }
            if action_state.just_released(&PlayerAction::MainThrusterBig)
                || action_state.just_released(&PlayerAction::MainThrusterMedium)
                || action_state.just_released(&PlayerAction::MainThrusterSmall)
            {
                if let Ok(sink) = sound_controller.get_single() {
                    sink.pause();
                }
            }
        } else {
            if let Ok(sink) = sound_controller.get_single() {
                sink.pause();
            }
        }
    }
}

fn pause_all_sound_effect_system(
    air_scape_sound_controller: Query<&AudioSink, With<AirScapeSoundEffect>>,
    thruster_sound_controller: Query<&AudioSink, With<ThrusterSoundEffect>>,
) {
    if let Ok(air_scape_sink) = air_scape_sound_controller.get_single() {
        air_scape_sink.pause();
    }
    if let Ok(thruster_sink) = thruster_sound_controller.get_single() {
        thruster_sink.pause();
    }
}
