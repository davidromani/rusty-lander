use bevy::prelude::*;

use crate::movement::Acceleration;
use crate::movement::Velocity;
use crate::movement::MovingObjectBundle;
use crate::asset_loader::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 20.);
// const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 0.5);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_spaceship_system)
            .add_systems(Update, spaceship_movement_controls_system)
        ;
    }
}

// Systems
fn spawn_spaceship_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Player
    ));
}

fn spaceship_movement_controls_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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

// Components
#[derive(Component)]
struct Player;
