use crate::game::Scores;
use crate::spaceship::{Player, INITIAL_SPACESHIP_POSITION};
use avian2d::prelude::{GravityScale, LinearVelocity};
use bevy::prelude::*;

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    Init,
    Setup,
    Menu,
    Game,
    Instructions,
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

fn transition_game_setup_to_running_system(
    mut state: ResMut<NextState<GameState>>,
    mut scores: ResMut<Scores>,
    mut spaceship_transform_query: Query<&mut Transform, With<Player>>,
    mut spaceship_linear_velocity_query: Query<&mut LinearVelocity, With<Player>>,
    mut spaceship_visibility_query: Query<&mut Visibility, With<Player>>,
    mut spaceship_gravity_query: Query<&mut GravityScale, With<Player>>,
) {
    info!("transitioning from GameState::Setup to -> GameState::Landing");
    let Ok(mut spaceship_transform) = spaceship_transform_query.get_single_mut() else {
        return;
    };
    let Ok(mut spaceship_linear_velocity) = spaceship_linear_velocity_query.get_single_mut() else {
        return;
    };
    let Ok(mut spaceship_gravity) = spaceship_gravity_query.get_single_mut() else {
        return;
    };
    spaceship_transform.translation.x = INITIAL_SPACESHIP_POSITION.x;
    spaceship_transform.translation.y = INITIAL_SPACESHIP_POSITION.y;
    spaceship_linear_velocity.x = 120.0;
    spaceship_linear_velocity.y = 0.0;
    scores.gravity += 0.1;
    spaceship_gravity.0 = scores.gravity;
    let mut spaceship_visibility = spaceship_visibility_query.single_mut();
    *spaceship_visibility = Visibility::Visible;
    state.set(GameState::Landing);
}
