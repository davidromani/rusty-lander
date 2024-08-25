use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::movement::*;

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
fn spawn_spaceship_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        CharacterControllerBundle::new(Collider::rectangle(60.0, 32.0)).with_movement(
            1250.0,
            0.92,
            600.0,
            (180.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.0),
        SpriteBundle {
            texture: scene_assets.lander.clone(),
            transform: Transform::from_xyz(0.0, 300.0, 2.0),
            ..default()
        },
        Player
    ));
}

// Components
#[derive(Component)]
struct Player;
