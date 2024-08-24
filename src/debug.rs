use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_hello_world_system);
    }
}

// Systems
fn print_hello_world_system() {
    println!("Hello 'Rusty Lander' World! from DebugPlugin");
}
