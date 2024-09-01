use bevy::prelude::*;
use bevy::input::common_conditions::*;
use bevy::sprite::*;

use crate::game::Scores;

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_fuel_bar_system)
            .add_systems(Update, handle_fire_big_booster_key_pressed_system.run_if(input_pressed(KeyCode::Digit2)))
            .add_systems(Update, handle_fire_medium_booster_key_pressed_system.run_if(input_pressed(KeyCode::KeyW)))
            .add_systems(Update, handle_fire_small_booster_key_pressed_system.run_if(input_pressed(KeyCode::KeyS)))
            .add_systems(Update, handle_fire_small_booster_key_pressed_system.run_if(input_pressed(KeyCode::KeyA)))
            .add_systems(Update, handle_fire_small_booster_key_pressed_system.run_if(input_pressed(KeyCode::KeyD)))
            .add_systems(Update, update_fuel_bar_system)
        ;
    }
}

// Systems
fn spawn_fuel_bar_system(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-500.0, -340.0, 3.0)),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                color: Color::srgb(0.19, 0.10, 0.84),
                custom_size: Some(Vec2::new(1000.0, 15.0)),
                ..default()
            },
            ..default()
        },
        FuelBar,
    ));
}

fn handle_fire_big_booster_key_pressed_system(mut scores: ResMut<Scores>) {
    if scores.fuel_quantity >= 0.0 {
        scores.fuel_quantity -= 1.0;
    }
}

fn handle_fire_medium_booster_key_pressed_system(mut scores: ResMut<Scores>) {
    if scores.fuel_quantity >= 0.0 {
        scores.fuel_quantity -= 0.5;
    }
}

fn handle_fire_small_booster_key_pressed_system(mut scores: ResMut<Scores>) {
    if scores.fuel_quantity >= 0.0 {
        scores.fuel_quantity -= 0.2;
    }
}

fn update_fuel_bar_system(mut query: Query<&mut Sprite, With<FuelBar>>, scores: Res<Scores>) {
    let Ok(mut sprite) = query.get_single_mut() else {
        return;
    };
    sprite.custom_size = Some(Vec2::new(scores.fuel_quantity, 15.0));
}

// Components
#[derive(Component, Debug)]
struct FuelBar;
