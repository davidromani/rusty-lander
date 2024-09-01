use bevy::prelude::*;
use bevy::input::common_conditions::*;
use bevy::app::AppExit;
use std::f32::consts::TAU;

use crate::asset_loader::SceneAssets;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(GameState { is_playing: IS_PLAYING });
        app
            .insert_resource(Scores { score: 0, hi_score: 0, fuel_quantity: 1000.0 })
            .add_systems(PostStartup, spawn_background_image_system) // runs only once at Startup sequence
            .add_systems(Update, handle_exit_key_pressed_system.run_if(input_just_pressed(KeyCode::Escape))) // main App looper
            .add_systems(Update, rotate_background_image_system)
        ; 
    }
}

// Systems
fn spawn_background_image_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.background.clone(),
            ..default()
        },
        Background,
        Rotatable { speed: 0.001 }
    ));
}

fn rotate_background_image_system(mut query: Query<(&mut Transform, &Rotatable), With<Background>>, timer: Res<Time>) {
    let Ok((mut transform, background)) = query.get_single_mut() else {
        return;
    };
    // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
    // and then multiplied by delta_seconds which is the time that passed last frame.
    // In other words. Speed is equal to the amount of rotations per second.
    transform.rotate_z(background.speed * TAU * timer.delta_seconds());
}

fn handle_exit_key_pressed_system(mut exit: EventWriter<AppExit>) {
    info!("exit key has been pressed");
    exit.send(AppExit::Success);
}

// Components
#[derive(Component)]
struct Background;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

// Resources (global scope allocated data)

#[derive(Resource, Debug)]
pub struct Scores {
    pub score: i16,
    pub hi_score: i16,
    pub fuel_quantity: f32,
}
