use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_collider_gen::avian2d::single_convex_polyline_collider_translated;
use leafwing_input_manager::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::game::Scores;
use crate::movement::*;
use crate::state::AppState;

pub const INITIAL_SPACESHIP_POSITION: Vec3 = Vec3::new(-300.0, 300.0, 2.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_systems(OnEnter(AppState::Game), spawn_spaceship_system);
    }
}

// Systems
fn spawn_spaceship_system(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    image_assets: Res<Assets<Image>>,
    scores: Res<Scores>,
) {
    let input_map = InputMap::new([
        (PlayerAction::MainThrusterBig, KeyCode::Digit2),
        (PlayerAction::MainThrusterBig, KeyCode::Space),
        (PlayerAction::MainThrusterMedium, KeyCode::KeyW),
        (PlayerAction::MainThrusterMedium, KeyCode::ArrowUp),
        (PlayerAction::MainThrusterSmall, KeyCode::KeyS),
        (PlayerAction::MainThrusterSmall, KeyCode::ArrowDown),
        (PlayerAction::LeftThruster, KeyCode::KeyA),
        (PlayerAction::LeftThruster, KeyCode::ArrowLeft),
        (PlayerAction::RightThruster, KeyCode::KeyD),
        (PlayerAction::RightThruster, KeyCode::ArrowRight),
    ]);
    let sprite_image_handle = scene_assets.lander.clone();
    let sprite_image = image_assets.get(&sprite_image_handle);
    let collider = single_convex_polyline_collider_translated(sprite_image.unwrap()).unwrap();
    commands.spawn((
        StateScoped(AppState::Game),
        CharacterControllerBundle::new(collider).with_movement(
            550.0, // before 1250.0
            0.965, // before 0.92
            4.9,   // before 60.0
        ),
        Friction::ZERO,
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(scores.gravity),
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform::from_translation(INITIAL_SPACESHIP_POSITION),
            ..default()
        },
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        Player,
    ));
}

// Actions
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Idle,
    LeftThruster,
    RightThruster,
    MainThrusterBig,
    MainThrusterMedium,
    MainThrusterSmall,
}

// Components
#[derive(Component, Debug)]
pub struct Player;

#[derive(Component)]
pub struct VerticalThrusterEffect;

#[derive(Component)]
pub struct LeftHorizontalThrusterEffect;

#[derive(Component)]
pub struct RightHorizontalThrusterEffect;

#[derive(Component)]
pub struct ThrusterSoundEffect;

#[derive(Component)]
pub struct AirScapeSoundEffect;
