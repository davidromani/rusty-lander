use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::game::Scores;
use crate::spaceship::Player;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_hello_world_system)
            .add_systems(
                Update,
                handle_debug_key_pressed_system.run_if(input_just_pressed(KeyCode::Digit1)),
            );
    }
}

// Systems
fn print_hello_world_system() {
    info!("Hello 'Rusty Lander' World!");
    // warn!("Entity {:?} · Component {:?}", entity, component);
}

fn handle_debug_key_pressed_system(
    mut query: Query<&LinearVelocity, With<Player>>,
    scores: Res<Scores>,
) {
    info!("Debug key 1 has been pressed");
    info!(
        "Current score = {:?} · Hi score = {:?} · Fuel = {:?}",
        scores.score, scores.hi_score, scores.fuel_quantity
    );
    let Ok(linear_velocity) = query.get_single_mut() else {
        return;
    };
    info!("Current LinearVelocity: {:?}", linear_velocity);
}
