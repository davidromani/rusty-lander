use crate::collider::{PLATFORM_10X_CENTER, PLATFORM_2X_CENTER, PLATFORM_5X_CENTER};
use crate::game::InGameSet;
use crate::gizmos::PROXIMITY_RADIUS;
use crate::spaceship::Player;
use crate::state::GameState;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, (spawn_camera_system, spawn_debug_ui_system))
            .add_systems(
                Update,
                (
                    add_or_remove_player_camera_components_depending_on_nearest_platform_system,
                    move_camera_position_to_nearest_platform_system,
                )
                    .run_if(in_state(GameState::Landing))
                    .in_set(InGameSet::Physics),
            )
            .add_systems(
                Update,
                detect_game_camera_close_to_platforms_removals_system
                    .run_if(in_state(GameState::Landing))
                    .in_set(InGameSet::SpeedBar),
            );
    }
}

// Systems
fn spawn_camera_system(mut commands: Commands) {
    commands.spawn((
        GameCamera,
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[0, 1]),
    ));
    commands.spawn((
        ControllersCamera,
        Camera2dBundle {
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[1, 2]),
    ));
}

fn add_or_remove_player_camera_components_depending_on_nearest_platform_system(
    mut commands: Commands,
    mut spaceship_query: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((entity, transform)) = spaceship_query.get_single_mut() else {
        return;
    };
    if transform.translation.distance(PLATFORM_2X_CENTER) < PROXIMITY_RADIUS {
        commands.entity(entity).insert(GameCameraCloseTo2xPlatform);
    } else {
        commands
            .entity(entity)
            .remove::<GameCameraCloseTo2xPlatform>();
    }
    if transform.translation.distance(PLATFORM_5X_CENTER) < PROXIMITY_RADIUS {
        commands.entity(entity).insert(GameCameraCloseTo5xPlatform);
    } else {
        commands
            .entity(entity)
            .remove::<GameCameraCloseTo5xPlatform>();
    }
    if transform.translation.distance(PLATFORM_10X_CENTER) < PROXIMITY_RADIUS {
        commands.entity(entity).insert(GameCameraCloseTo10xPlatform);
    } else {
        commands
            .entity(entity)
            .remove::<GameCameraCloseTo10xPlatform>();
    }
}

fn move_camera_position_to_nearest_platform_system(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<GameCamera>>,
    spaceship_close_to_2x_platform_query: Query<Entity, Added<GameCameraCloseTo2xPlatform>>,
    spaceship_close_to_5x_platform_query: Query<Entity, Added<GameCameraCloseTo5xPlatform>>,
    spaceship_close_to_10x_platform_query: Query<Entity, Added<GameCameraCloseTo10xPlatform>>,
) {
    let (mut transform, mut projection) = camera_query.single_mut();
    for _entity in spaceship_close_to_2x_platform_query.iter() {
        projection.scale /= 1.25;
        transform.translation = PLATFORM_2X_CENTER;
    }
    for _entity in spaceship_close_to_5x_platform_query.iter() {
        projection.scale /= 1.25;
        transform.translation = PLATFORM_5X_CENTER;
    }
    for _entity in spaceship_close_to_10x_platform_query.iter() {
        projection.scale /= 1.25;
        transform.translation = PLATFORM_10X_CENTER;
    }
}

fn detect_game_camera_close_to_platforms_removals_system(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<GameCamera>>,
    mut game_camera_close_to_2x_platform_removals: RemovedComponents<GameCameraCloseTo2xPlatform>,
    mut game_camera_close_to_5x_platform_removals: RemovedComponents<GameCameraCloseTo5xPlatform>,
    mut game_camera_close_to_10x_platform_removals: RemovedComponents<GameCameraCloseTo10xPlatform>,
) {
    let (mut transform, mut projection) = camera_query.single_mut();
    for _entity in game_camera_close_to_2x_platform_removals.read() {
        projection.scale *= 1.25;
        transform.translation = Vec3::ZERO;
    }
    for _entity in game_camera_close_to_5x_platform_removals.read() {
        projection.scale *= 1.25;
        transform.translation = Vec3::ZERO;
    }
    for _entity in game_camera_close_to_10x_platform_removals.read() {
        projection.scale *= 1.25;
        transform.translation = Vec3::ZERO;
    }
}

fn spawn_debug_ui_system(mut commands: Commands) {
    #[cfg(debug_assertions)]
    commands.spawn((
        PerfUiRoot {
            values_col_width: Some(160.0),
            ..Default::default()
        },
        (
            PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
            PerfUiWidgetBar::new(PerfUiEntryFPSWorst::default()),
            PerfUiWidgetBar::new(PerfUiEntryEntityCount::default()),
            PerfUiWidgetBar::new(PerfUiEntryCpuUsage::default()),
            PerfUiWidgetBar::new(PerfUiEntryMemUsage::default()),
        ),
        (
            PerfUiEntryCursorPosition::default(),
            PerfUiEntryWindowResolution::default(),
        ),
    ));
}

// Components
#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct ControllersCamera;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct GameCameraCloseTo2xPlatform;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct GameCameraCloseTo5xPlatform;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct GameCameraCloseTo10xPlatform;
