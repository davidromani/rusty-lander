mod handler;
mod interaction;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::asset_loader::UiAssets;
use crate::game::{Scores, FUEL_QUANTITY};
use crate::state::{AppState, GameState};
use crate::MAIN_TITLE;

pub use handler::*;
pub use interaction::*;

const PRIMARY_COLOR: Color = Color::srgb(0.54, 0.13, 0.07);
const SECONDARY_COLOR: Color = Color::srgb(0.45, 0.68, 0.74);
const GREY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const DARK_GREY_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BLACK_COLOR: Color = Color::srgb(0.04, 0.04, 0.04);

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    MenuUp,
    MenuDown,
    Accept,
    PauseUnpause,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Init), setup)
            .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(OnEnter(AppState::Instructions), spawn_instructions_menu)
            .add_systems(OnEnter(AppState::Credits), spawn_credits_menu)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (
                    main_menu_input_system,
                    menu_selection_system,
                    menu_blink_system,
                ),
            )
            .add_systems(
                Update,
                game_menu_input_system.run_if(in_state(AppState::Game)),
            );
    }
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<MenuAction>::new([
        (MenuAction::Accept, KeyCode::Enter),
        (MenuAction::PauseUnpause, KeyCode::KeyP),
        (MenuAction::MenuUp, KeyCode::KeyW),
        (MenuAction::MenuUp, KeyCode::ArrowUp),
        (MenuAction::MenuDown, KeyCode::KeyS),
        (MenuAction::MenuDown, KeyCode::ArrowDown),
    ]);
    input_map.insert(MenuAction::PauseUnpause, GamepadButtonType::Start);
    input_map.insert(MenuAction::Accept, GamepadButtonType::South);
    // insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

fn spawn_main_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: MAIN_TITLE.to_string(),
        main_text_color: SECONDARY_COLOR,
        main_text_blink: false,
        selected_id: 0,
        entries: vec![
            "Play".into(),
            "Instructions".into(),
            "Credits".into(),
            "Exit".into(),
        ],
    }
    .spawn(&mut commands, assets.font_kenvector.clone());
    commands.entity(entity).insert(StateScoped(AppState::Menu));
}

fn spawn_game_over_menu(
    mut commands: Commands,
    assets: ResMut<UiAssets>,
    mut score: ResMut<Scores>,
) {
    let entity = MenuHandler {
        main_text: "Game Over".into(),
        main_text_color: Color::srgb_u8(0xAA, 0x22, 0x22),
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font_kenvector.clone());
    commands
        .entity(entity)
        .insert(StateScoped(GameState::GameOver));
    score.fuel_quantity = FUEL_QUANTITY;
}

fn spawn_pause_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "Pause".into(),
        main_text_color: Color::srgb_u8(0xF8, 0xE4, 0x73),
        main_text_blink: true,
        selected_id: 0,
        entries: vec!["Resume".into(), "Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font_kenvector.clone());
    commands
        .entity(entity)
        .insert(StateScoped(GameState::Paused));
}

fn spawn_instructions_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "".into(),
        main_text_color: GREY_COLOR,
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font_kenvector.clone());
    commands
        .entity(entity)
        .insert(StateScoped(AppState::Instructions));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::Instructions),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(25.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Land on one of three platforms. Vertical velocity must come within the yellow area of the scale. The score, according to the velocity, is multiplied by the number under platform. You'll be refueled on a successful landing. Every landing the gravity increases.",
                    TextStyle {
                        font: assets.font_kenvector.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(25.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Press A or arrow LEFT key to push spaceship right.\nPress D or arrow RIGHT key to push spaceship left.\nPress 2 or SPACE key to enable a big thrust up.\nPress W or arrow UP key to enable a medium thrust up.\nPress S or arrow DOWN key to enable a small thrust up.",
                    TextStyle {
                        font: assets.font_kenvector.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },));
        });
}

fn spawn_credits_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "".into(),
        main_text_color: PRIMARY_COLOR,
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font_kenvector.clone());
    commands
        .entity(entity)
        .insert(StateScoped(AppState::Credits));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::Credits),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Code",
                    TextStyle {
                        font: assets.font_kenvector.clone(),
                        font_size: 50.0,
                        color: PRIMARY_COLOR,
                    },
                ),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                text: Text::from_sections([
                    TextSection::new(
                        "David Romaní ",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "(github.com/davidromani)",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 20.0,
                            color: GREY_COLOR,
                        },
                    ),
                ]),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Acknowledgements",
                    TextStyle {
                        font: assets.font_kenvector.clone(),
                        font_size: 50.0,
                        color: PRIMARY_COLOR,
                    },
                ),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                text: Text::from_sections([
                    TextSection::new(
                        "Boris Boutillier ",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 35.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "(github.com/BorisBoutillier)",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 20.0,
                            color: GREY_COLOR,
                        },
                    ),
                ]),
                ..default()
            },));
        });
}
