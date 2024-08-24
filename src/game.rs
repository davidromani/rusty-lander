use bevy::prelude::*;

const DEBUG_MODE: bool = true;
const IS_PLAYING: bool = true;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
        app.insert_resource(GameState { is_playing: IS_PLAYING });
        app.add_systems(Startup, (add_people_system, print_hello_world_system, setup_system)); // runs only once at Startup sequence
        app.add_systems(Update, greet_people_system); // main App looper
    }
}

// Systems
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/logo.png"),
        ..default()
    });
}

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

// Components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Resources
#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource, Debug)]
pub struct GameState {
    pub is_playing: bool,
}
