mod game;
mod camera;

use bevy::prelude::*;
use game::GamePlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .run()
    ;
}
