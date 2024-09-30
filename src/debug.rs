use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::game::Scores;
use crate::MAIN_TITLE;

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
    info!("Hello '{}' World!", MAIN_TITLE.to_string());
}

fn handle_debug_key_pressed_system(scores: Res<Scores>) {
    info!("Debug key 1 has been pressed");
    info!(
        "Current score = {:?} · Hi score = {:?} · Fuel = {:?} · Gravity = {:?}",
        scores.score, scores.hi_score, scores.fuel_quantity, scores.gravity
    );
}
