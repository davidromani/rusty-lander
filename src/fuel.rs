use bevy::prelude::*;

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_fuel_bar_system)
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
        FuelBar { quantity: 1000 }
    ));
}

// Components
#[derive(Component)]
struct FuelBar {
    pub quantity: i32,
}
