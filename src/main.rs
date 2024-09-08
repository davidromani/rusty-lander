mod asset_loader;
mod camera;
mod collider;
mod debug;
mod fuel;
mod game;
mod movement;
mod spaceship;
mod speedometer;
mod state;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use asset_loader::AssetsLoaderPlugin;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use debug::DebugPlugin;
use fuel::FuelPlugin;
use game::GamePlugin;
use movement::CharacterControllerPlugin;
use spaceship::SpaceshipPlugin;
use speedometer::SpeedometerPlugin;
use state::StatesPlugin;

fn main() {
    App::new()
        // Bevy & Avian2D plugins
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusty Lander".to_string(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDebugPlugin::default(),
        ))
        // Resources
        .insert_resource(Gravity(Vector::NEG_Y * 98.0))
        // Custom plugins
        .add_plugins(StatesPlugin) // update
        .add_plugins(AssetsLoaderPlugin) // startup
        .add_plugins(CameraPlugin) // startup
        .add_plugins(DebugPlugin) // startup
        .add_plugins(FuelPlugin) // startup & update
        .add_plugins(SpeedometerPlugin) // startup & update
        .add_plugins(ColliderPlugin) // post startup
        .add_plugins(SpaceshipPlugin) // post startup
        .add_plugins(GamePlugin) // post startup & update
        .add_plugins(CharacterControllerPlugin) // update
        .run();
}
