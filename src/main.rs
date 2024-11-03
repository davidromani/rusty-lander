#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::single_match,
    clippy::collapsible_else_if,
    non_upper_case_globals
)]

mod asset_loader;
mod audio;
mod camera;
mod collider;
mod debug;
mod explosion;
mod fuel;
mod game;
mod gizmos;
mod menu;
mod movement;
mod particles_thruster;
mod spaceship;
mod speedometer;
mod state;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use leafwing_input_manager::plugin::InputManagerPlugin;
use std::string::ToString;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

use asset_loader::AssetsLoaderPlugin;
use audio::AudioPlugin;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use debug::DebugPlugin;
use explosion::ExplosionPlugin;
use fuel::FuelPlugin;
use game::{GamePlugin, InGameSet, WorldBoundsVertices2D};
use gizmos::GizmosPlugin;
use menu::{MenuAction, MenuPlugin};
use movement::CharacterControllerPlugin;
use particles_thruster::ParticlesThrusterPlugin;
use spaceship::SpaceshipPlugin;
use speedometer::SpeedometerPlugin;
use state::StatesPlugin;

const MAIN_TITLE: &str = "Rusty Lander";
const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();
    // Bevy, Avian2d & Leafwing plugins
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: MAIN_TITLE.to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: false,
                    focused: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default().with_length_unit(20.0),
        InputManagerPlugin::<MenuAction>::default(),
    ));
    // Enable Avian2d debug renders when compiled in debug mode
    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());
    // Resources
    app.insert_resource(Gravity(Vector::NEG_Y * 58.0))
        .insert_resource(get_world_bounds_resource_2d());
    // System ordering
    app.configure_sets(
        FixedUpdate,
        (
            InGameSet::Collisions,
            InGameSet::Physics,
            InGameSet::SpeedBar,
        )
            .chain(),
    );
    // Custom plugins
    #[cfg(debug_assertions)]
    app.add_plugins(GizmosPlugin);
    app.add_plugins(StatesPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(FuelPlugin)
        .add_plugins(ParticlesThrusterPlugin)
        .add_plugins(SpeedometerPlugin)
        .add_plugins(ColliderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(CharacterControllerPlugin)
        .add_plugins(ExplosionPlugin)
        .run();
}

fn get_world_bounds_resource_2d() -> WorldBoundsVertices2D {
    let mut current_point: Vec2 = Vec2::new(0.0, 0.0);
    let mut world_bounds_resource_2d = WorldBoundsVertices2D { data: vec![] };
    let path = "assets/svg/landscape.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        Command::Move(_position, params) => {
                            current_point.x = params[0];
                            current_point.y = params[1];
                            world_bounds_resource_2d.data.push(current_point);
                        }
                        Command::CubicCurve(_position, params) => {
                            let chunks = params.chunks_exact(2);
                            for chunk in chunks {
                                let mut next_point = current_point;
                                next_point.x += chunk[0];
                                next_point.y -= chunk[1];
                                world_bounds_resource_2d.data.push(next_point);
                                current_point = next_point;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    world_bounds_resource_2d
}
