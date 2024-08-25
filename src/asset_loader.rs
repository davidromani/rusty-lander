use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::{asset::LoadState, prelude::*};
use bevy::color::palettes::css;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_collider_gen::avian2d::single_heightfield_collider_translated;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SceneAssets>()
            .init_state::<SceneAssetState>()
            .add_systems(PreStartup, load_assets_system)
            .add_systems(
                Update, 
                (
                    check_assets.run_if(in_state(SceneAssetState::Loading)),
                    //intialize_landscape_system.run_if(in_state(SceneAssetState::Loaded)),
                )
            )
            .add_systems(OnEnter(SceneAssetState::Loaded), intialize_landscape_system)
        ;
    }
}

// Systems
pub fn check_assets(
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneAssets>,
    mut state: ResMut<NextState<SceneAssetState>>,
) {
    // return if the background isn't loaded
    if Some(LoadState::Loaded) != asset_server.get_load_state(&scene_assets.background) {
        return;
    }
    // return if the landscape isn't loaded
    if Some(LoadState::Loaded) != asset_server.get_load_state(&scene_assets.landscape) {
        return;
    }
    // return if the lander isn't loaded
    if Some(LoadState::Loaded) != asset_server.get_load_state(&scene_assets.lander) {
        return;
    }
    // all assets have loaded
    state.set(SceneAssetState::Loaded)
}

pub fn load_assets_system(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        background: asset_server.load("background_space.png"),
        // landscape: asset_server.load("landscape.png"),
        landscape: asset_server.load("terrain.png"),
        lander: asset_server.load("lander.png"),
    }
}

fn intialize_landscape_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scene_assets: Res<SceneAssets>,
    image_assets: Res<Assets<Image>>
) {
    // bottom world bounds
    commands.spawn((
        Collider::rectangle(1200.0, 5.0),
        RigidBody::Static,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(1200.0, 5.0)).into(),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(0.0, -355.0, 1.0),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));

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
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

// Resources
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub background: Handle<Image>,
    pub landscape: Handle<Image>,
    pub lander: Handle<Image>,
}

// States
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SceneAssetState {
    #[default]
    Loading,
    Loaded,
}
