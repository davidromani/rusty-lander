use avian2d::prelude::{LinearVelocity, Physics, PhysicsTime};
use bevy::prelude::*;

use crate::game::Resettable;
use crate::spaceship::{Player, INITIAL_SPACESHIP_POSITION};

// States
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

// Events
#[derive(Event)]
pub struct TenSecondsEvent;

#[derive(Resource)]
pub struct TenSecondsTimer(pub Timer);

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_sub_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            .add_event::<TenSecondsEvent>()
            .add_systems(
                Update,
                (
                    transition_app_setup_to_menu_system.run_if(in_state(AppState::Setup)),
                    transition_game_setup_to_running_system.run_if(in_state(GameState::Setup)),
                    check_ten_seconds_timer.run_if(in_state(GameState::Landing)),
                ),
            );
    }
}

// Systems
fn transition_app_setup_to_menu_system(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Menu);
}

fn transition_game_setup_to_running_system(
    resettable_text_query: Query<Entity, With<Resettable>>,
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    mut spaceship_transform_query: Query<&mut Transform, With<Player>>,
    mut spaceship_linear_velocity_query: Query<&mut LinearVelocity, With<Player>>,
    mut spaceship_visibility_query: Query<&mut Visibility, With<Player>>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    physics_time.unpause();
    for entity in resettable_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let Ok(mut spaceship_transform) = spaceship_transform_query.get_single_mut() else {
        return;
    };
    let Ok(mut spaceship_linear_velocity) = spaceship_linear_velocity_query.get_single_mut() else {
        return;
    };
    spaceship_transform.translation.x = INITIAL_SPACESHIP_POSITION.x;
    spaceship_transform.translation.y = INITIAL_SPACESHIP_POSITION.y;
    spaceship_linear_velocity.x = 120.0;
    spaceship_linear_velocity.y = 0.0;
    let mut spaceship_visibility = spaceship_visibility_query.single_mut();
    *spaceship_visibility = Visibility::Visible;
    state.set(GameState::Landing);
}

fn check_ten_seconds_timer(
    time: Res<Time>,
    mut timer: ResMut<TenSecondsTimer>,
    mut events: EventWriter<TenSecondsEvent>,
) {
    // Update the timer
    if timer.0.tick(time.delta()).just_finished() {
        // Send the event once the timer completes
        events.send(TenSecondsEvent);
        info!("10 seconds timer ended");
    }
}
