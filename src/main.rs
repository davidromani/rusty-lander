mod game;
mod camera;
mod movement;

use bevy::prelude::*;
use game::GamePlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .run()
    ;
}
