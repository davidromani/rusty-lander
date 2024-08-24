use bevy::prelude::*;

// const DEBUG_MODE: bool = true;
const IS_PLAYING: bool = true;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { is_playing: IS_PLAYING });
        app.add_systems(Startup, setup_system); // runs only once at Startup sequence
        // app.add_systems(Update, greet_people_system); // main App looper
    }
}

// Systems
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background_space.png"),
        ..default()
    });
}

// Resources (global scope allocated data)
#[derive(Resource, Debug)]
pub struct GameState {
    pub is_playing: bool,
}
