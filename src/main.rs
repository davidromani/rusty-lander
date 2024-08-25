mod asset_loader;
mod camera;
mod collider;
mod debug;
mod game;
mod movement;
mod spaceship;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use asset_loader::AssetsLoaderPlugin;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
use movement::CharacterControllerPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy & Avian2D plugins
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDebugPlugin::default(),
        ))
        // Resources
        .insert_resource(Gravity(Vector::NEG_Y * 9.8))
        // Sets
        .configure_sets(Startup, (AppSet::First, AppSet::Second).chain())
        // Custom plugins
        .add_plugins(AssetsLoaderPlugin) // startup
        .add_plugins(CameraPlugin) // startup
        .add_plugins(DebugPlugin) // startup
        .add_plugins(ColliderPlugin) // post startup
        .add_plugins(SpaceshipPlugin) // post startup
        .add_plugins(GamePlugin) // post startup & update
        .add_plugins(CharacterControllerPlugin) // update
        .run()
    ;
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppSet {
    First,
    Second,
}
