use crate::game::InGameSet;
use crate::menu::BLACK_COLOR;
use crate::spaceship::Player;
use crate::state::{AppState, GameState};
use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::*;

pub struct SpeedometerPlugin;

impl Plugin for SpeedometerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_speed_bar_system)
            .add_systems(
                FixedUpdate,
                update_speed_bar_black_indicator_system
                    .run_if(in_state(GameState::Landing))
                    .in_set(InGameSet::SpeedBar),
            );
    }
}

// Systems
fn spawn_speed_bar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // green bar range
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(485.0, 0.0, 3.0)),
            sprite: Sprite {
                color: Color::srgb(0.32, 0.75, 0.03),
                custom_size: Some(Vec2::new(15.0, 600.0)),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(2),
    ));
    // yellow range
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(485.0, -17.5, 4.0)),
            sprite: Sprite {
                color: Color::srgb(0.77, 0.84, 0.11),
                custom_size: Some(Vec2::new(15.0, 35.0)),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(2),
    ));
    // black indicator
    commands.spawn((
        StateScoped(AppState::Game),
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(485.0, 0.0, 6.0)),
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 2.0))),
            material: materials.add(BLACK_COLOR),
            ..default()
        },
        SpeedBarBlackIndicator,
        RenderLayers::layer(2),
    ));
}

fn update_speed_bar_black_indicator_system(
    mut query_speed_bar_black_indicators: Query<&mut Transform, With<SpeedBarBlackIndicator>>,
    mut query_player_linear_velocities: Query<&LinearVelocity, With<Player>>,
) {
    let Ok(mut black_indicator) = query_speed_bar_black_indicators.get_single_mut() else {
        return;
    };
    let Ok(linear_velocity) = query_player_linear_velocities.get_single_mut() else {
        return;
    };
    if linear_velocity.y < 300.0 && linear_velocity.y > -300.0 {
        black_indicator.translation.y = linear_velocity.y;
    }
}

// Components
#[derive(Component, Debug)]
pub struct SpeedBarBlackIndicator;
