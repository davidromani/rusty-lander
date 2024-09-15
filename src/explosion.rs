use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::asset_loader::SceneAssets;
use crate::fuel::FuelBar;
use crate::game::Scores;
use crate::speedometer::SpeedBarBlackIndicator;
use crate::state::{AppState, GameState};

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
    mut commands: Commands,
    mut event_reader: EventReader<SpawnExplosionEvent>,
    scene_assets: Res<SceneAssets>,
) {
    for event in event_reader.read() {
        let (texture, start_size, end_scale, duration) = (
            scene_assets.explosion.clone(),
            Vec2::new(211.0, 195.0),
            3.5,
            2.5,
        );
        commands.spawn((
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
            Explosion {
                timer: Timer::from_seconds(duration, TimerMode::Once),
                start_scale: 0.75,
                end_scale,
            },
            StateScoped(AppState::Game),
        ));
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
    mut fuel_bar_query: Query<Entity, With<FuelBar>>,
    mut speed_bar_black_indicator_query: Query<Entity, With<SpeedBarBlackIndicator>>,
    mut commands: Commands,
    mut scores: ResMut<Scores>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !event_reader.is_empty() {
        scores.fuel_quantity -= 100.0;
        if scores.fuel_quantity <= 0.0 {
            game_state.set(GameState::GameOver);
        } else {
            let Ok(fuel_bar) = fuel_bar_query.get_single_mut() else {
                return;
            };
            let Ok(speed_bar_black_indicator) = speed_bar_black_indicator_query.get_single_mut()
            else {
                return;
            };
            commands.entity(fuel_bar).despawn_recursive();
            commands
                .entity(speed_bar_black_indicator)
                .despawn_recursive();
            game_state.set(GameState::Setup);
        }
    }
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
