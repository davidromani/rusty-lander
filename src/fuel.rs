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
        FuelBar { fuel_quantity: 1000.0 },
    ));
}

fn handle_fire_big_booster_key_pressed_system(mut query: Query<(&mut FuelBar, &mut Sprite), With<FuelBar>>, scores: Res<Scores>) {
    let Ok((mut fuel_bar, mut sprite)) = query.get_single_mut() else {
        return;
    };
    fuel_bar.fuel_quantity -= 1.0;
    sprite.custom_size = Some(Vec2::new(fuel_bar.fuel_quantity, 15.0));
    warn!("fuel_bar {:?} · sprite {:?} · scores {:?}", fuel_bar, sprite, scores);
}

// Components
#[derive(Component, Debug)]
struct FuelBar {
    pub fuel_quantity: f32,
}
