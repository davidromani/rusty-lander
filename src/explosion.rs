use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::asset_loader::SceneAssets;
use crate::state::AppState;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosionEvent>().add_systems(
            Update,
            (animate_explosion_system, catch_explosion_event_system),
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
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed = time.delta();
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished() {
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

// Events
#[derive(Event)]
pub struct SpawnExplosionEvent {
    pub x: f32,
    pub y: f32,
}

// Components
#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}
