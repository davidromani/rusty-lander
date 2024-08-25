use bevy::prelude::*;

use crate::AppSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (print_hello_world_system).in_set(AppSet::Second));
    }
}

// Systems
fn print_hello_world_system() {
    info!("Hello 'Rusty Lander' World!");
    // info!("Entity {:?} · Component {:?}", entity, component);
}
