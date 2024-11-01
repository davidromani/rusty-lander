use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::TAU;

use crate::asset_loader::{AudioAssets, SceneAssets};
use crate::audio::{MusicBeginSoundEffect, MusicPlayingSoundEffect};
use crate::game::Scores;
use crate::spaceship::Player;
use crate::state::{AppState, GameState};

const CAMERA_DECAY_RATE: f32 = 0.9; // Adjust this for smoother or snappier decay
const TRAUMA_DECAY_SPEED: f32 = 0.5; // How fast trauma decays
const TRAUMA_INCREMENT: f32 = 1.0; // Increment of trauma per frame when holding space

// screen_shake parameters, maximum addition by frame not actual maximum overall values
const MAX_ANGLE: f32 = 0.5;
const MAX_OFFSET: f32 = 500.0;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosionEvent>()
            .add_event::<FinishedExplosionEvent>()
            .add_systems(
                Update,
                (
                    animate_explosion_system.run_if(in_state(GameState::Crashed)),
                    catch_explosion_event_system.run_if(in_state(GameState::Crashed)),
                    catch_finished_explosion_event_system.run_if(in_state(GameState::Crashed)),
                ),
            );
    }
}

fn catch_explosion_event_system(
    scene_assets: Res<SceneAssets>,
    audio_assets: Res<AudioAssets>,
    music_playing_controller: Query<&AudioSink, With<MusicPlayingSoundEffect>>,
    music_begin_controller: Query<&AudioSink, With<MusicBeginSoundEffect>>,
    time: Res<Time>,
    mut commands: Commands,
    mut spaceship_visibility_query: Query<&mut Visibility, With<Player>>,
    mut events_reader: EventReader<SpawnExplosionEvent>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    for event in events_reader.read() {
        if let Ok(sink) = music_playing_controller.get_single() {
            sink.pause();
        }
        if let Ok(sink) = music_begin_controller.get_single() {
            sink.pause();
        }
        let (texture, start_size, end_scale, duration) = (
            scene_assets.explosion.clone(),
            Vec2::new(211.0, 195.0),
            3.5,
            2.5,
        );
        commands.spawn((
            StateScoped(AppState::Game),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(start_size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 10.0),
                    ..default()
                },
                texture,
                ..default()
            },
            AudioBundle {
                source: audio_assets.ship_explosion.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            },
            Explosion {
                timer: Timer::from_seconds(duration, TimerMode::Once),
                start_scale: 0.75,
                end_scale,
            },
        ));
        let mut spaceship_visibility = spaceship_visibility_query.single_mut();
        *spaceship_visibility = Visibility::Hidden;
        let screen_shake_clone = screen_shake.clone();
        screen_shake.start_shake(
            MAX_ANGLE,
            MAX_OFFSET,
            screen_shake_clone.trauma + TRAUMA_INCREMENT * time.delta_seconds(),
            Vec2 { x: 0.0, y: 0.0 },
        ); // final_position should be your current player position

        break;
    }
}

fn animate_explosion_system(
    time: Res<Time>,
    mut commands: Commands,
    mut finished_explosion_events: EventWriter<FinishedExplosionEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed = time.delta();
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished() {
            finished_explosion_events.send(FinishedExplosionEvent {});
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat(
                explosion.start_scale
                    + (explosion.end_scale - explosion.start_scale)
                        * (explosion.timer.elapsed_secs()
                            / explosion.timer.duration().as_secs_f32()),
            );
            transform.rotate_z(0.0025 * TAU * explosion.timer.duration().as_secs_f32());
        }
    }
}

fn catch_finished_explosion_event_system(
    event_reader: EventReader<FinishedExplosionEvent>,
    mut scores: ResMut<Scores>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !event_reader.is_empty() {
        scores.fuel_quantity -= 100.0;
        if scores.fuel_quantity <= 0.0 {
            scores.score = 0;
            game_state.set(GameState::GameOver);
        } else {
            game_state.set(GameState::Setup);
        }
    }
}

fn screen_shake(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query: Query<(&mut Camera, &mut Transform)>,
) {
    let mut rng = ChaCha8Rng::from_entropy();
    let shake = screen_shake.trauma * screen_shake.trauma;
    let angle = (screen_shake.max_angle * shake).to_radians() * rng.gen_range(-1.0..1.0);
    let offset_x = screen_shake.max_offset * shake * rng.gen_range(-1.0..1.0);
    let offset_y = screen_shake.max_offset * shake * rng.gen_range(-1.0..1.0);

    if shake > 0.0 {
        for (mut camera, mut transform) in query.iter_mut() {
            // Position
            let sub_view = camera.sub_camera_view.as_mut().unwrap();
            let target = sub_view.offset
                + Vec2 {
                    x: offset_x,
                    y: offset_y,
                };
            sub_view
                .offset
                .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_seconds());

            // Rotation
            let rotation = Quat::from_rotation_z(angle);
            transform.rotation = transform
                .rotation
                .interpolate_stable(&(transform.rotation.mul_quat(rotation)), CAMERA_DECAY_RATE);
        }
    } else {
        // return camera to the latest position of player (it's fixed in this example case)
        if let Ok((mut camera, mut transform)) = query.get_single_mut() {
            let sub_view = camera.sub_camera_view.as_mut().unwrap();
            let target = screen_shake.latest_position.unwrap();
            sub_view
                .offset
                .smooth_nudge(&target, 1.0, time.delta_seconds());
            transform.rotation = transform.rotation.interpolate_stable(&Quat::IDENTITY, 0.1);
        }
    }
    // Decay the trauma over time
    screen_shake.trauma -= TRAUMA_DECAY_SPEED * time.delta_seconds();
    screen_shake.trauma = screen_shake.trauma.clamp(0.0, 1.0);
}

// Events
#[derive(Event)]
pub struct SpawnExplosionEvent {
    pub x: f32,
    pub y: f32,
}

#[derive(Event)]
pub struct FinishedExplosionEvent {}

// Components
#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}

// Resources
#[derive(Resource, Clone)]
struct ScreenShake {
    max_angle: f32,
    max_offset: f32,
    trauma: f32,
    latest_position: Option<Vec2>,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            max_angle: 0.0,
            max_offset: 0.0,
            trauma: 0.0,
            latest_position: Some(Vec2::default()),
        }
    }
}

impl ScreenShake {
    fn start_shake(&mut self, max_angle: f32, max_offset: f32, trauma: f32, final_position: Vec2) {
        self.max_angle = max_angle;
        self.max_offset = max_offset;
        self.trauma = trauma.clamp(0.0, 1.0);
        self.latest_position = Some(final_position);
    }
}
