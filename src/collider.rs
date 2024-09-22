use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::color::palettes::css;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{ecs::query::Has, prelude::*};

use crate::explosion::SpawnExplosionEvent;
use crate::game::{SpaceshipJustLandedEvent, WorldBoundsVertices};
use crate::spaceship::Player;
use crate::state::{AppState, GameState};
use crate::{asset_loader::SceneAssets, movement::ReadyToLand};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), initialize_landscape_system)
            .add_systems(
                Update,
                player_landed_collisions_system.run_if(in_state(GameState::Landing)),
            );
    }
}

// Systems
fn initialize_landscape_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    landscape_world_bounds_vertices: Res<WorldBoundsVertices>,
    scene_assets: Res<SceneAssets>,
) {
    // world bounds collider
    let world_bounds_vertices = vec![
        Vector::new(-500.0, 10360.0),
        Vector::new(-500.0, -300.0),
        Vector::new(492.0, -300.0),
        Vector::new(492.0, 10360.0),
    ];
    let world_bounds_polyline = Collider::polyline(world_bounds_vertices, None);
    commands.spawn((
        StateScoped(AppState::Game),
        RigidBody::Static,
        world_bounds_polyline,
        DebugRender::default().with_collider_color(css::INDIAN_RED.into()),
    ));
    // platform x2
    commands.spawn((
        StateScoped(AppState::Game),
        Collider::rectangle(185.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(132.0, 164.0, 1.0),
            ..default()
        },
        Platform { factor: 2 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // platform x5
    commands.spawn((
        StateScoped(AppState::Game),
        Collider::rectangle(200.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(-269.0, -222.0, 1.0),
            ..default()
        },
        Platform { factor: 5 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // platform x10
    commands.spawn((
        StateScoped(AppState::Game),
        Collider::rectangle(120.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(60.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(248.0, -102.0, 1.0),
            ..default()
        },
        Platform { factor: 10 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // land image
    let sprite_image_handle = scene_assets.landscape.clone();
    commands.spawn((
        StateScoped(AppState::Game),
        RigidBody::Static,
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
    // land collider
    let collider = Collider::polyline(landscape_world_bounds_vertices.data.clone(), None);
    commands.spawn((
        StateScoped(AppState::Game),
        collider,
        RigidBody::Static,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-495.0, 306.0, 1.0),
                scale: Vec3::new(1.45, 1.45, 1.0),
                ..default()
            },
            ..default()
        },
        DebugRender::default().with_collider_color(css::STEEL_BLUE.into()),
    ));
}

fn player_landed_collisions_system(
    query: Query<
        (
            &CollidingEntities,
            &LinearVelocity,
            &Transform,
            Has<ReadyToLand>,
        ),
        With<Player>,
    >,
    platforms_query: Query<&Platform>,
    mut game_state: ResMut<NextState<GameState>>,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
    mut spaceship_just_landed_spawn_events: EventWriter<SpaceshipJustLandedEvent>,
) {
    for (colliding_entities, linear_velocity, transform, is_ready_to_land) in &query {
        if !colliding_entities.is_empty() {
            if !is_ready_to_land {
                info!("Lander is not ready to land. Crash!");
                explosion_spawn_events.send(SpawnExplosionEvent {
                    x: transform.translation.x,
                    y: transform.translation.y,
                });
                game_state.set(GameState::Crashed);
            } else {
                for &colliding_entity in colliding_entities.iter() {
                    if let Ok(platform) = platforms_query.get(colliding_entity) {
                        info!(
                            "Landed in platform factor {:?} with linear velocity {:?}",
                            platform.factor, linear_velocity.y
                        );
                        spaceship_just_landed_spawn_events.send(SpaceshipJustLandedEvent {
                            platform: platform.clone(),
                            linear_velocity: linear_velocity.clone(),
                        });
                        game_state.set(GameState::Landed);
                    } else {
                        info!("Landed outside a platform");
                        explosion_spawn_events.send(SpawnExplosionEvent {
                            x: transform.translation.x,
                            y: transform.translation.y,
                        });
                        game_state.set(GameState::Crashed);
                    }
                }
            }
        }
    }
}

// Components
#[derive(Component, Clone, Debug)]
pub struct Platform {
    pub factor: i32,
}
