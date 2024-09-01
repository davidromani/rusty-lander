use bevy::prelude::*;
use bevy::sprite::*;

pub struct SpeedometerPlugin;

impl Plugin for SpeedometerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_speed_bar_system)
            .add_systems(Update, update_fuel_bar_system)
        ;
    }
}

// Systems
fn spawn_speed_bar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    // black indicator
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 2.0))),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(620.0, 0.0, 5.0)),
            ..default()
        },
        SpeedBarBlackIndicator,
    ));
}

fn update_fuel_bar_system(mut query: Query<&mut Transform, With<SpeedBarBlackIndicator>>) {
    let Ok(mut black_indicator) = query.get_single_mut() else {
        return;
    };
    black_indicator.translation.y -= 0.1;
}

// Components
#[derive(Component, Debug)]
struct SpeedBarBlackIndicator;