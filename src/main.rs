mod asset_loader;
mod camera;
mod collider;
mod debug;
mod game;
mod movement;
mod spaceship;

use avian2d::prelude::*;
use bevy::prelude::*;

use asset_loader::AssetsLoaderPlugin;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy & Avian plugins
        .add_plugins((
            DefaultPlugins, 
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        // custom plugins
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ColliderPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .run()
    ;
}
