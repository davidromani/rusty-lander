use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
        app.add_plugins(PerfUiPlugin);
        app.add_systems(Startup, spawm_camera_system);
    }
}

fn spawm_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Instead of using `PerfUiCompleteBundle`,
    // spawn an entity with `PerfUiRoot` + whatever entries you want!
    commands.spawn((
        PerfUiRoot {
            // set a fixed width to make all the bars line up
            values_col_width: Some(160.0),
            ..Default::default()
        },
        // when we have lots of entries, we have to group them
        // into tuples, because of Bevy Rust syntax limitations
        (
            PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
            PerfUiWidgetBar::new(PerfUiEntryFPSWorst::default()),
            PerfUiWidgetBar::new(PerfUiEntryEntityCount::default()),
            PerfUiWidgetBar::new(PerfUiEntryCpuUsage::default()),
            PerfUiWidgetBar::new(PerfUiEntryMemUsage::default()),
        ),
        (
            PerfUiEntryFixedTimeStep::default(),
            PerfUiEntryClock::default(),
        ),
        (
            PerfUiEntryCursorPosition::default(),
            PerfUiEntryWindowResolution::default(),
            PerfUiEntryWindowScaleFactor::default(),
            PerfUiEntryWindowMode::default(),
            PerfUiEntryWindowPresentMode::default(),
        ),
    ));
}
