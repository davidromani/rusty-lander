use bevy::prelude::*;

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    Init,
    Setup,
    Menu,
    Game,
    Credits,
}

#[derive(SubStates, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
#[source(AppState=AppState::Game)]
pub enum GameState {
    #[default]
    Setup,
    Landing,
    Paused,
    Landed,
    Crashed,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_sub_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            .add_systems(
                Update,
                (
                    transition_app_setup_to_menu_system.run_if(in_state(AppState::Setup)),
                    transition_game_setup_to_running_system.run_if(in_state(GameState::Setup)),
                ),
            );
    }
}

// Systems
fn transition_app_setup_to_menu_system(mut state: ResMut<NextState<AppState>>) {
    info!("transitioning from AppState::Setup to -> AppState::Menu");
    state.set(AppState::Menu);
}

fn transition_game_setup_to_running_system(mut state: ResMut<NextState<GameState>>) {
    // TODO set score = 0
    // TODO translate spaceship sprite to initial position
    // TODO hide texts
    info!("transitioning from GameState::Setup to -> GameState::Landing");
    state.set(GameState::Landing);
}
