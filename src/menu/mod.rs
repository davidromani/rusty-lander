mod handler;
mod interaction;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::asset_loader::UiAssets;
use crate::state::{AppState, GameState};
use crate::MAIN_TITLE;

pub use handler::*;
pub use interaction::*;

// List of user actions associated to menu/ui interaction
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    // In menus move up the highlighted entry
    MenuUp,
    // In menus move down the highlighted entry
    MenuDown,
    // In menus, select highlighted entry
    Accept,
    // During gameplay, pause the game.
    // Also, directly unpause the game when in the pause screen.
    PauseUnpause,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), setup)
            .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(OnEnter(AppState::Credits), spawn_credits_menu)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnEnter(GameState::Crashed), spawn_game_over_menu)
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
        (MenuAction::PauseUnpause, KeyCode::Escape),
        (MenuAction::MenuUp, KeyCode::KeyW),
        (MenuAction::MenuUp, KeyCode::ArrowUp),
        (MenuAction::MenuDown, KeyCode::KeyS),
        (MenuAction::MenuDown, KeyCode::ArrowDown),
    ]);
    input_map.insert(MenuAction::PauseUnpause, GamepadButtonType::Start);
    input_map.insert(MenuAction::Accept, GamepadButtonType::South);
    // Insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

fn spawn_main_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: MAIN_TITLE.to_string(),
        main_text_color: Color::srgb(0.0, 0.7, 0.7),
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Play".into(), "Credits".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands.entity(entity).insert(StateScoped(AppState::Menu));
}

fn spawn_game_over_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "Game Over".into(),
        main_text_color: Color::srgb_u8(0xAA, 0x22, 0x22),
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands
        .entity(entity)
        .insert(StateScoped(GameState::Crashed));
}

fn spawn_pause_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "Pause".into(),
        main_text_color: Color::srgb_u8(0xF8, 0xE4, 0x73),
        main_text_blink: true,
        selected_id: 0,
        entries: vec!["Resume".into(), "Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands
        .entity(entity)
        .insert(StateScoped(GameState::Paused));
}

fn spawn_credits_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = MenuHandler {
        main_text: "".into(),
        main_text_color: Color::srgb(0.0, 0.7, 0.7),
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
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
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                text: Text::from_section(
                    "Code",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 50.0,
                        color: Color::srgb(0.0, 0.7, 0.7),
                    },
                ),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
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
                            color: Color::srgb(0.5, 0.5, 0.5),
                        },
                    ),
                ]),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                text: Text::from_section(
                    "Assets",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 50.0,
                        color: Color::srgb(0.0, 0.7, 0.7),
                    },
                ),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                text: Text::from_sections([
                    TextSection::new(
                        "Kenney Vleugels ",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 35.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "(www.kenney.nl)",
                        TextStyle {
                            font: assets.font_fira.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.5, 0.5, 0.5),
                        },
                    ),
                ]),
                ..default()
            },));
            parent.spawn((TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                text: Text::from_sections([TextSection::new(
                    "Pablo Roman Andrioli",
                    TextStyle {
                        font: assets.font_fira.clone(),
                        font_size: 35.0,
                        color: Color::WHITE,
                    },
                )]),
                ..default()
            },));
        });
}
