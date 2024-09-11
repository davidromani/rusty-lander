use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_collider_gen::avian2d::single_convex_polyline_collider_translated;
use leafwing_input_manager::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::movement::*;
use crate::state::GameState;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_systems(OnEnter(GameState::Setup), spawn_spaceship_system);
    }
}

// Systems
fn spawn_spaceship_system(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    image_assets: Res<Assets<Image>>,
) {
    let input_map = InputMap::new([
        (PlayerAction::MainThrusterBig, KeyCode::Digit2),
        (PlayerAction::MainThrusterBig, KeyCode::Digit9),
        (PlayerAction::MainThrusterMedium, KeyCode::KeyW),
        (PlayerAction::MainThrusterMedium, KeyCode::KeyO),
        (PlayerAction::MainThrusterMedium, KeyCode::Space),
        (PlayerAction::MainThrusterSmall, KeyCode::KeyS),
        (PlayerAction::MainThrusterSmall, KeyCode::KeyL),
        (PlayerAction::LeftThruster, KeyCode::KeyA),
        (PlayerAction::LeftThruster, KeyCode::ArrowLeft),
        (PlayerAction::RightThruster, KeyCode::KeyD),
        (PlayerAction::RightThruster, KeyCode::ArrowRight),
    ]);
    let sprite_image_handle = scene_assets.lander.clone();
    let sprite_image = image_assets.get(&sprite_image_handle);
    let collider = single_convex_polyline_collider_translated(sprite_image.unwrap()).unwrap();
    commands.spawn((
        CharacterControllerBundle::new(collider).with_movement(
            550.0, // before 1250.0
            0.97,  // before 0.92
            4.9,   // before 60.0
            (180.0 as Scalar).to_radians(),
        ),
        Friction::ZERO
            .with_static_coefficient(0.15)
            .with_combine_rule(CoefficientCombine::Min),
        Restitution::PERFECTLY_ELASTIC.with_combine_rule(CoefficientCombine::Multiply),
        ColliderDensity(2.0),
        GravityScale(1.0),
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform::from_xyz(-200.0, 300.0, 2.0),
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
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct VerticalThrusterEffect;

#[derive(Component)]
pub struct LeftHorizontalThrusterEffect;

#[derive(Component)]
pub struct RightHorizontalThrusterEffect;
