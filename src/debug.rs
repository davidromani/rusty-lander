use bevy::{prelude::*, input::common_conditions::input_just_pressed};

use crate::AppSet;
use crate::game::Scores;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (print_hello_world_system).in_set(AppSet::Second))
            .add_systems(Update, handle_debug_key_pressed_system.run_if(input_just_pressed(KeyCode::Digit1)))
        ;
    }
}

// Systems
fn print_hello_world_system() {
    info!("Hello 'Rusty Lander' World!");
    // warn!("Entity {:?} · Component {:?}", entity, component);
}

fn handle_debug_key_pressed_system(scores: Res<Scores>) {
    info!("Debug key 1 has been pressed");
        info!("Current score = {:?} · Hi score = {:?} · Fuel = {:?}", scores.score, scores.hi_score, scores.fuel_quantity);
}
