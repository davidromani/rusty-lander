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
        // Bevy & Avian plugins
        .add_plugins((
            DefaultPlugins,
            // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
            // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDebugPlugin::default(),
        ))
        // Resources
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        // Custom plugins
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ColliderPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(CharacterControllerPlugin)
        .add_plugins(SpaceshipPlugin)
        .run()
    ;
}
