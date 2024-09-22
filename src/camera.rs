use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, (spawn_camera_system)); //, spawn_debug_ui_system));
    }
}

// Systems
fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
