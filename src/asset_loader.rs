use bevy::prelude::*;

use crate::AppSet;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SceneAssets>()
            .add_systems(Startup, (load_assets_system).in_set(AppSet::First))
        ;
    }
}

// Systems
pub fn load_assets_system(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        background: asset_server.load("background_space.png"),
        //landscape: asset_server.load("landscape.png"),
        landscape: asset_server.load("terrain.png"),
        lander: asset_server.load("lander.png"),
    }
}

// Resources
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub background: Handle<Image>,
    pub landscape: Handle<Image>,
    pub lander: Handle<Image>,
}
