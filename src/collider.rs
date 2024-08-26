use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::color::palettes::css;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_collider_gen::avian2d::single_heightfield_collider_translated;

use crate::asset_loader::{SceneAssetState, SceneAssets};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SceneAssetState::Loaded), intialize_landscape_system)
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
    // world bounds collider
    let vertices = vec![
        Vector::new(-630.0, 360.0),
        Vector::new(-630.0, -300.0),
        Vector::new(630.0, -300.0),
        Vector::new(630.0, 360.0)
    ];
    let polyline = Collider::polyline(vertices, None);
    commands.spawn((
        RigidBody::Static,
        polyline,
        DebugRender::default().with_collider_color(css::INDIAN_RED.into()),
    ));

    // platform x2
    commands.spawn((
        Collider::rectangle(100.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(100.0, 55.0, 1.0),
            ..default()
        },
        Platform { factor: 2 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));

    // platform x5
    commands.spawn((
        Collider::rectangle(100.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(-260.0, -220.0, 1.0),
            ..default()
        },
        Platform { factor: 5 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));

    // platform x10
    commands.spawn((
        Collider::rectangle(60.0, 8.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(60.0, 8.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(200.0, -95.0, 1.0),
            ..default()
        },
        Platform { factor: 5 },
        DebugRender::default().with_collider_color(css::SPRING_GREEN.into()),
    ));

    // land
    let sprite_image_handle = scene_assets.landscape.clone();
    info!("sprite_image_handle {:?}", sprite_image_handle);
    let sprite_image = image_assets.get(&sprite_image_handle);
    info!("sprite_image {:?}", sprite_image);
    let collider = single_heightfield_collider_translated(sprite_image.unwrap());
    commands.spawn((
        collider,
        RigidBody::Static,
        SpriteBundle {
            texture: sprite_image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, -260.0, 1.0),
                //scale: Vec3::fromnew(2.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

// Components
#[derive(Component)]
struct Platform {
    factor: i8,
}
