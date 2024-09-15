use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::menu::{MenuAction, MenuHandler};
use crate::state::{AppState, GameState};

pub fn main_menu_input_system(
    app_state: ResMut<State<AppState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    menu: Query<&MenuHandler>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Ok(menu) = menu.get_single() {
        if menu_action_state.just_pressed(&MenuAction::Accept) {
            if app_state.get() == &AppState::Menu {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Game);
                    }
                    1 => {
                        next_app_state.set(AppState::Credits);
                    }
                    _ => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
            if app_state.get() == &AppState::Credits {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
        }
    }
}

pub fn game_menu_input_system(
    game_state: ResMut<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu: Query<&MenuHandler>,
) {
    if menu_action_state.just_pressed(&MenuAction::PauseUnpause) {
        if game_state.get() == &GameState::Landing {
            next_game_state.set(GameState::Paused);
        }
        if game_state.get() == &GameState::Paused {
            next_game_state.set(GameState::Landing);
        }
    }
    if let Ok(menu) = menu.get_single() {
        if menu_action_state.just_pressed(&MenuAction::Accept) {
            if game_state.get() == &GameState::Paused {
                match menu.selected_id {
                    0 => {
                        next_game_state.set(GameState::Landing);
                    }
                    1 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
            if game_state.get() == &GameState::GameOver {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
        }
    }
}
