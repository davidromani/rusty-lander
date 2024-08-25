use avian2d::prelude::*;
use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, intialize_land_system)
        ;
    }
}

// Systems
fn intialize_land_system(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        RigidBody::Static,
        Collider::circle(250.0),
        SpriteBundle {
            texture: scene_assets.landscape.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    ));
}
