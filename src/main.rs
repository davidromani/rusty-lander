mod asset_loader;
mod camera;
mod collider;
mod debug;
mod explosion;
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
use explosion::ExplosionPlugin;
use fuel::FuelPlugin;
use game::GamePlugin;
use menu::{MenuAction, MenuPlugin};
use movement::CharacterControllerPlugin;
use particles_thruster::ParticlesThrusterPlugin;
use spaceship::SpaceshipPlugin;
use speedometer::SpeedometerPlugin;
use state::StatesPlugin;

const MAIN_TITLE: &str = "Rusty Lander";

fn main() {
    let mut app = App::new();
    // Bevy, Avian2d & Leafwing plugins
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: MAIN_TITLE.to_string(),
                resizable: false,
                focused: true,
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
    app.insert_resource(Gravity(Vector::NEG_Y * 58.0));
    // Custom plugins
    app.add_plugins(StatesPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(FuelPlugin)
        /*.add_plugins(ParticlesThrusterPlugin)*/
        .add_plugins(SpeedometerPlugin)
        .add_plugins(ColliderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(GamePlugin)
        /*.add_plugins(CharacterControllerPlugin)*/
        .add_plugins(ExplosionPlugin)
        .run();
}
