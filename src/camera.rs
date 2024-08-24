use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawm_camera_system);
    }
}

fn spawm_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
