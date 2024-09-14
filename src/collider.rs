use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_collider_gen::avian2d::single_heightfield_collider_translated;

use crate::explosion::SpawnExplosionEvent;
use crate::spaceship::Player;
use crate::state::GameState;
use crate::{
    asset_loader::SceneAssets,
    movement::{CharacterController, Grounded, ReadyToLand},
};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Landing), initialize_landscape_system)
            .add_systems(
                Update,
                (crash_collisions_system, print_player_landed_system)
                    .run_if(in_state(GameState::Landing)),
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

fn crash_collisions_system(
    query: Query<(Entity, &CollidingEntities, &Transform), (With<Player>, Without<Grounded>)>,
    mut commands: Commands,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (entity, colliding_entities, transform) in &query {
        if !colliding_entities.is_empty() {
            explosion_spawn_events.send(SpawnExplosionEvent {
                x: transform.translation.x,
                y: transform.translation.y,
            });
            commands.entity(entity).despawn_recursive();
            game_state.set(GameState::Crashed);
        }
    }
}

fn print_player_landed_system(
    query: Query<(Entity, &CollidingEntities, &CharacterController), With<ReadyToLand>>,
    platforms_query: Query<&Platform>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (entity, colliding_entities, player) in &query {
        if !colliding_entities.is_empty() {
            println!(
                "{:?} is colliding with the following entities: {:?}",
                entity, colliding_entities
            );
            println!("Player is ReadyToLand {:?}", player);
            for &colliding_entity in colliding_entities.iter() {
                if let Ok(platform) = platforms_query.get(colliding_entity) {
                    println!("Landed in platform factor {:?}", platform.factor);
                }
            }
            game_state.set(GameState::Landed);
        }
    }
}

// Components
#[derive(Component, Debug)]
pub struct Platform {
    factor: i8,
}
