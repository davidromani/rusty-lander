use bevy::prelude::*;

pub struct SpeedometerPlugin;

impl Plugin for SpeedometerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_speed_bar_system)
        ;
    }
}

// Systems
fn spawn_speed_bar_system(mut commands: Commands) {
    // green bar range
    commands.spawn(
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(620.0, 0.0, 3.0)),
            sprite: Sprite {
                color: Color::srgb(0.32, 0.75, 0.03),
                custom_size: Some(Vec2::new(15.0, 600.0)),
                ..default()
            },
            ..default()
        }
    );
    // yellow range
    commands.spawn(
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(620.0, 0.0, 4.0)),
            sprite: Sprite {
                color: Color::srgb(0.77, 0.84, 0.11),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            ..default()
        }
    );
    // yellow range
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(620.0, 0.0, 5.0)),
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(15.0, 2.0)),
                ..default()
            },
            ..default()
        },
        SpeedBar,
    ));
}

// Components
#[derive(Component, Debug)]
struct SpeedBar;
