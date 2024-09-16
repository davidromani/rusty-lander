use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::color::palettes::css;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{ecs::query::Has, prelude::*};
use bevy_collider_gen::avian2d::single_heightfield_collider_translated;

use crate::asset_loader::UiAssets;
use crate::explosion::SpawnExplosionEvent;
use crate::game::{Scores, TextScoringAfterLanding, FUEL_QUANTITY};
use crate::spaceship::Player;
use crate::state::GameState;
use crate::{asset_loader::SceneAssets, movement::ReadyToLand};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Landing), initialize_landscape_system)
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
    scene_assets: Res<SceneAssets>,
    image_assets: Res<Assets<Image>>,
) {
    // world bounds collider
    let world_bounds_vertices = vec![
        Vector::new(-630.0, 360.0),
        Vector::new(-630.0, -300.0),
        Vector::new(630.0, -300.0),
        Vector::new(630.0, 360.0),
    ];
    let world_bounds_polyline = Collider::polyline(world_bounds_vertices, None);
    commands.spawn((
        RigidBody::Static,
        world_bounds_polyline,
        DebugRender::default().with_collider_color(css::INDIAN_RED.into()),
    ));
    // platform x2
    commands.spawn((
        Collider::rectangle(100.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(100.0, 55.0, 1.0),
            ..default()
        },
        Platform { factor: 2 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // platform x5
    commands.spawn((
        Collider::rectangle(100.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(-260.0, -220.0, 1.0),
            ..default()
        },
        Platform { factor: 5 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // platform x10
    commands.spawn((
        Collider::rectangle(60.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(60.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(200.0, -95.0, 1.0),
            ..default()
        },
        Platform { factor: 10 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));
    // land
    let sprite_image_handle = scene_assets.landscape.clone();
    let sprite_image = image_assets.get(&sprite_image_handle);
    let collider = single_heightfield_collider_translated(sprite_image.unwrap());
    commands.spawn((
        collider,
        RigidBody::Static,
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, -240.0, 1.0),
                scale: Vec3::new(2.5, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

fn player_landed_collisions_system(
    query: Query<(Entity, &CollidingEntities, &Transform, Has<ReadyToLand>), With<Player>>,
    platforms_query: Query<&Platform>,
    assets: ResMut<UiAssets>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut scores: ResMut<Scores>,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
) {
    for (entity, colliding_entities, transform, is_ready_to_land) in &query {
        if !colliding_entities.is_empty() {
            if !is_ready_to_land {
                println!("Lander is not ready to land. Crash!");
                explosion_spawn_events.send(SpawnExplosionEvent {
                    x: transform.translation.x,
                    y: transform.translation.y,
                });
                commands.entity(entity).despawn_recursive(); // TODO set invisible
                game_state.set(GameState::Crashed);
            } else {
                for &colliding_entity in colliding_entities.iter() {
                    if let Ok(platform) = platforms_query.get(colliding_entity) {
                        println!("Landed in platform factor {:?}", platform.factor);
                        let mut new_score =
                            platform.factor * scores.get_available_fuel_quantity() as i16;
                        scores.score += new_score;
                        if scores.hi_score < scores.score {
                            scores.hi_score = scores.score;
                        }
                        commands.spawn((
                            TextScoringAfterLanding,
                            TextBundle::from_section(
                                (scores.get_available_fuel_quantity() as i16).to_string()
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
                        if new_score > FUEL_QUANTITY as i16 {
                            new_score = FUEL_QUANTITY as i16;
                        }
                        scores.fuel_quantity += new_score as f32;
                        game_state.set(GameState::Landed);
                    } else {
                        println!("Landed outside a platform");
                        explosion_spawn_events.send(SpawnExplosionEvent {
                            x: transform.translation.x,
                            y: transform.translation.y,
                        });
                        commands.entity(entity).despawn_recursive(); // TODO set invisible
                        game_state.set(GameState::Crashed);
                    }
                }
            }
        }
    }
}

// Components
#[derive(Component, Debug)]
pub struct Platform {
    factor: i16,
}
