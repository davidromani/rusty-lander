use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Update, print_hello_world_system)
        .run()
    ;
}

// systems
fn print_hello_world_system() {
    println!("Hello 'Rusty Lander' World!");
}
