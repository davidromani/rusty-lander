use bevy::{asset::LoadState, prelude::*};

use crate::state::AppState;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_state::<SceneAssetState>()
            .add_systems(
                OnEnter(AppState::Init),
                (load_assets_system, load_ui_assets_system),
            )
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
    mut app_state: ResMut<NextState<AppState>>,
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
    // all assets have been loaded
    state.set(SceneAssetState::Loaded);
    app_state.set(AppState::Setup);
}

pub fn load_assets_system(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        background: asset_server.load("background_space.png"),
        // landscape: asset_server.load("landscape.png"),
        landscape: asset_server.load("terrain.png"),
        lander: asset_server.load("spaceship.png"),
        // lander: asset_server.load("lander.png"),
        explosion: asset_server.load("explosion.png"),
    }
}

fn load_ui_assets_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        // font_kenvector: asset_server.load("fonts/kenvector_future.ttf"),
        font_kenvector: asset_server.load("fonts/vt323_regular.ttf"),
        font_fira: asset_server.load("fonts/fira_sans_bold.ttf"),
        font_vt323: asset_server.load("fonts/vt323_regular.ttf"),
    });
}

// Resources
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub background: Handle<Image>,
    pub landscape: Handle<Image>,
    pub lander: Handle<Image>,
    pub explosion: Handle<Image>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font_kenvector: Handle<Font>,
    pub font_fira: Handle<Font>,
    pub font_vt323: Handle<Font>,
}

// States
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SceneAssetState {
    #[default]
    Loading,
    Loaded,
}
