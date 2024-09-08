use bevy::{asset::LoadState, prelude::*};

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_state::<SceneAssetState>()
            .add_systems(PreStartup, load_assets_system)
            .add_systems(
                Update,
                check_assets.run_if(in_state(SceneAssetState::Loading)),
            );
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
        lander: asset_server.load("spaceship.png"),
        // lander: asset_server.load("lander.png"),
    }
}

// Resources
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub background: Handle<Image>,
    pub landscape: Handle<Image>,
    pub lander: Handle<Image>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub font_fira: Handle<Font>,
}

// States
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SceneAssetState {
    #[default]
    Loading,
    Loaded,
}
