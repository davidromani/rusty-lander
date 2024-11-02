use crate::collider::{PLATFORM_10X_CENTER, PLATFORM_2X_CENTER, PLATFORM_5X_CENTER};
use crate::game::InGameSet;
use crate::gizmos::PROXIMITY_RADIUS;
use crate::spaceship::Player;
use crate::state::GameState;
use bevy::prelude::*;
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
                add_or_remove_player_camera_components_depending_on_nearest_platform_system
                    .run_if(in_state(GameState::Landing))
                    .in_set(InGameSet::Physics),
            );
    }
}

// Systems
fn spawn_camera_system(mut commands: Commands) {
    commands.spawn((GameCamera, Camera2dBundle::default()));
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
#[component(storage = "SparseSet")]
struct GameCameraCloseTo2xPlatform;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct GameCameraCloseTo5xPlatform;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct GameCameraCloseTo10xPlatform;
