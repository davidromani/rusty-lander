use bevy::prelude::*;

// Main state enum, differentiating, Menu from Game 'scenes'
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
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
        app.init_state::<AppState>();
        app.enable_state_scoped_entities::<AppState>();
        app.add_sub_state::<GameState>();
        app.enable_state_scoped_entities::<GameState>();
        app.add_systems(
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
    state.set(AppState::Menu);
}

fn transition_game_setup_to_running_system(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Landing);
}
