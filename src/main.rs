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

fn greet_people_system(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hi commander {}!", name.0);
        }
    }
}

// components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// resources
#[derive(Resource)]
struct GreetTimer(Timer);

// plugins
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
        app.add_systems(Startup, (add_people_system, print_hello_world_system));
        app.add_systems(Update, greet_people_system);
    }
}
