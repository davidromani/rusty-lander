mod asset_loader;
mod camera;
mod collider;
mod debug;
mod fuel;
mod game;
mod menu;
mod movement;
mod particles_thruster;
mod spaceship;
mod speedometer;
mod state;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use std::string::ToString;

use asset_loader::AssetsLoaderPlugin;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use debug::DebugPlugin;
use fuel::FuelPlugin;
use game::GamePlugin;
use menu::MenuAction;
use menu::MenuPlugin;
use movement::CharacterControllerPlugin;
use particles_thruster::ParticlesThrusterPlugin;
use spaceship::SpaceshipPlugin;
use speedometer::SpeedometerPlugin;
use state::StatesPlugin;

const MAIN_TITLE: &str = "Rusty Lander";

fn main() {
    let mut app = App::new();
    // Bevy, Avian2d & Leafwing Input Manager plugins
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: MAIN_TITLE.to_string(),
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default().with_length_unit(20.0),
        InputManagerPlugin::<MenuAction>::default(),
    ));
    // Enable Avian2d debug renders when compiled in debug mode
    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());
    // Resources
    app.insert_resource(Gravity(Vector::NEG_Y * 98.0));
    // Custom plugins
    app.add_plugins(StatesPlugin) // update
        .add_plugins(MenuPlugin) // update
        .add_plugins(AssetsLoaderPlugin) // startup
        .add_plugins(CameraPlugin) // startup
        .add_plugins(DebugPlugin) // startup
        .add_plugins(FuelPlugin) // startup & update
        .add_plugins(ParticlesThrusterPlugin) // startup & update
        .add_plugins(SpeedometerPlugin) // startup & update
        .add_plugins(ColliderPlugin) // post startup
        .add_plugins(SpaceshipPlugin) // post startup
        .add_plugins(GamePlugin) // post startup & update
        .add_plugins(CharacterControllerPlugin) // update
        .run();
}
