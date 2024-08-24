mod game;
mod camera;
mod movement;
mod spaceship;

use bevy::prelude::*;
use game::GamePlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .run()
    ;
}
