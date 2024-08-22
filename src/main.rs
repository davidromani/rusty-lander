use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people_system)
        .add_systems(Update, print_hello_world_system)
        .run()
    ;
}

// systems
fn print_hello_world_system() {
    println!("Hello 'Rusty Lander' World!");
}

fn add_people_system(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
