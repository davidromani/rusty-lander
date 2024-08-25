use avian2d::{math::*, prelude::*};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::asset_loader::SceneAssets;
use crate::movement::*;

// const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 20.);
// const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 0.5);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_spaceship_system)
            //.add_systems(Update, spaceship_movement_controls_system)
        ;
    }
}

// Systems
fn spawn_spaceship_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::new(12.5, 20.0)).into(),
            material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0.0, 300.0, 10.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
            1250.0,
            0.92,
            400.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(0.5),
        /*
        SpriteBundle {
            texture: scene_assets.lander.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..default()
        },
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        */
        Player
    ));
}

/*
fn spaceship_movement_controls_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (transform, mut velocity) = query.single_mut();
    let mut movement = Vec3::ZERO;

    // left key
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("A");
        movement.x = -200.0;
    }
    // right key
    if keyboard_input.pressed(KeyCode::KeyD) {
        info!("D");
        movement.x = 200.0;
    }
    // up key
    if keyboard_input.pressed(KeyCode::KeyW) {
        info!("W");
        movement.y = 200.0;
    }
    // debug key
    if keyboard_input.just_released(KeyCode::Space) {
        info!("Transform {:?} · Velocity {:?}", transform, velocity);
    }

    velocity.value = movement;
}
*/

// Components
#[derive(Component)]
struct Player;
