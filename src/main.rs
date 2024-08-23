use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
//        .add_systems(Startup, add_people_system) // only run once
//        .add_systems(Update, (print_hello_world_system, greet_people_system)) // looper
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

fn greet_people_system(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hi commander {}!", name.0);
    }
}

// components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// plugins
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people_system);
        app.add_systems(Update, (print_hello_world_system, greet_people_system));
    }
}
