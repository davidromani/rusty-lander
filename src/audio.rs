use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::asset_loader::AudioAssets;
use crate::game::Scores;
use crate::spaceship::{AirScapeSoundEffect, PlayerAction};
use crate::state::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_audio_entities_system)
            .add_systems(
                Update,
                play_air_scape_sound_effect_system.run_if(in_state(GameState::Landing)),
            );
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
            ..default()
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
        }
    }
}
