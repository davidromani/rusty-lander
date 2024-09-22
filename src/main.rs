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

use avian2d::parry::na::DimAdd;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use leafwing_input_manager::plugin::InputManagerPlugin;
use std::string::ToString;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

use crate::game::WorldBoundsVertices;
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
    let mut current_point: Vec2 = Vec2::new(0.0, 0.0);
    let mut world_bounds_resource = WorldBoundsVertices { data: vec![] };
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
                            //println!("move points {:?}", params);
                            current_point.x = params[0];
                            current_point.y = params[1];
                            world_bounds_resource.data.push(current_point);
                        }
                        Command::CubicCurve(_position, params) => {
                            //println!("curve points len {:?}", params.len());
                            let mut chunks = params.chunks_exact(2);
                            for chunk in chunks {
                                //println!("item {:?} {:?}", chunk[0], chunk[1]);
                                let mut next_point = current_point;
                                next_point.x += chunk[0];
                                next_point.y -= chunk[1];
                                world_bounds_resource.data.push(next_point);
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
    let mut app = App::new();
    // Bevy, Avian2d & Leafwing plugins
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: MAIN_TITLE.to_string(),
                resolution: WindowResolution::new(1024.0, 720.0),
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
    app.insert_resource(Gravity(Vector::NEG_Y * 58.0))
        .insert_resource(world_bounds_resource);
    // Custom plugins
    app.add_plugins(StatesPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(AssetsLoaderPlugin)
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
