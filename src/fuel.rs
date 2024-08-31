use bevy::prelude::*;

use crate::game::Scores;

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_fuel_bar_system)
            .add_systems(Update, handle_fire_big_booster_key_pressed_system)
        ;
    }
}

// Systems
fn spawn_fuel_bar_system(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -320.0, 3.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1000.0, 15.0)),
                ..default()
            },
            ..default()
        },
        FuelBar { fuel_quantity: 10000 },
    ));
}

fn handle_fire_big_booster_key_pressed_system(mut query: Query<(&mut FuelBar, &mut Sprite)>, scores: Res<Scores>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Digit2) {
        for (mut fuel_bar, sprite) in &mut query { // TODO get only one queried object
            fuel_bar.fuel_quantity -= 1;
            warn!("fuel_bar {:?} · sprite {:?} · scores {:?}", fuel_bar, sprite, scores);
        }
    }
}

// Components
#[derive(Component, Debug)]
struct FuelBar {
    pub fuel_quantity: i32,
}
