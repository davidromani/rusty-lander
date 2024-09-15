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
            OnEnter(GameState::Setup),
            transition_game_setup_to_running_system,
        );
        app.add_systems(
            Update,
            (
                transition_app_setup_to_menu_system.run_if(in_state(AppState::Setup)),
                on_completed_transition_game_setup_to_running_system
                    .run_if(in_state(GameState::Setup)),
            ),
        );
    }
}

// Systems
fn transition_app_setup_to_menu_system(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Menu);
}

fn transition_game_setup_to_running_system(mut commands: Commands) {
    commands.spawn(OnCompletionTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn on_completed_transition_game_setup_to_running_system(
    time: Res<Time>,
    mut query: Query<&mut OnCompletionTimer>,
    mut state: ResMut<NextState<GameState>>,
) {
    for mut timer in &mut query {
        if timer.tick(time.delta()).just_finished() {
            info!("Entity timer just finished");
            state.set(GameState::Landing);
        }
    }
}

// Components
#[derive(Component, Deref, DerefMut)]
struct OnCompletionTimer(Timer);
