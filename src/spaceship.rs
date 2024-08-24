use bevy::prelude::*;

use crate::movement::Velocity;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship_system);
    }
}

// Systems
fn spawn_spaceship_system(mut commands: Commands) {
    commands.spawn((
        SceneBundle::default(),
        Velocity::new(Vec3::new(0., 0., 1.)),
    ));
}
