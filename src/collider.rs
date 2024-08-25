use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::color::palettes::css;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_collider_gen::avian2d::single_heightfield_collider_translated;

use crate::asset_loader::SceneAssets;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, intialize_landscape_system)
        ;
    }
}

// Systems
fn intialize_landscape_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scene_assets: Res<SceneAssets>,
    image_assets: Res<Assets<Image>>
) {
    // keep first
    commands.spawn((
        Collider::circle(100.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(100.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(0.0, -150.0, 1.0),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
    commands.spawn((
        Collider::rectangle(700.0, 15.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(700.0, 15.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(0.0, -345.0, 1.0),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));

    let sprite_image_handle = scene_assets.landscape.clone();
    info!("sprite_image_handle {:?}", sprite_image_handle);
    let sprite_image = image_assets.get(&sprite_image_handle);
    info!("sprite_image {:?}", sprite_image);
    if sprite_image.is_none() {
        return;
    }
    let collider = single_heightfield_collider_translated(sprite_image.unwrap());
    commands.spawn((
        collider,
        RigidBody::Static,
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}
