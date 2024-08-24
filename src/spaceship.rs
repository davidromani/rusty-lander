use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 0.5);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship_system);
    }
}

// Systems
fn spawn_spaceship_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        marker: Player,
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        sprite: SpriteBundle {
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    });
}

// Components
#[derive(Component)]
struct Player;

// Bundles
#[derive(Bundle)]
struct SpaceshipBundle {
    marker: Player,
    velocity: Velocity,
    sprite: SpriteBundle,
}
