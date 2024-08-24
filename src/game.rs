use bevy::prelude::*;
use bevy::input::common_conditions::*;
use bevy::app::AppExit;

// const DEBUG_MODE: bool = true;
// const IS_PLAYING: bool = true;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(GameState { is_playing: IS_PLAYING });
        app
            .add_systems(Startup, setup_system) // runs only once at Startup sequence
            .add_systems(Update, handle_exit_key_pressed_system.run_if(input_just_pressed(KeyCode::Escape))) // main App looper
        ; 
    }
}

// Systems
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background_space.png"),
        ..default()
    });
}

fn handle_exit_key_pressed_system(mut exit: EventWriter<AppExit>) {
    info!("exit key has been pressed");
    exit.send(AppExit::Success);
}

// Resources (global scope allocated data)
/*
#[derive(Resource, Debug)]
pub struct GameState {
    pub is_playing: bool,
}
*/