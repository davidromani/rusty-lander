use bevy::prelude::*;
use bevy::sprite::*;

use crate::asset_loader::UiAssets;
use crate::game::{Scores, FUEL_QUANTITY};
use crate::state::{AppState, GameState};

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Game),
            (spawn_fuel_bar_system, spawn_fuel_bar_text_system),
        )
        .add_systems(
            Update,
            update_fuel_bar_system.run_if(in_state(GameState::Landing)),
        );
    }
}

// Systems
fn spawn_fuel_bar_system(mut commands: Commands) {
    commands.spawn((
        StateScoped(AppState::Game),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-500.0, -340.0, 3.0)),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                color: Color::srgb(0.19, 0.10, 0.84),
                custom_size: Some(Vec2::new(FUEL_QUANTITY, 15.0)),
                ..default()
            },
            ..default()
        },
        FuelBar,
    ));
}

fn spawn_fuel_bar_text_system(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands.spawn((
        StateScoped(AppState::Game),
        TextBundle::from_section(
            "Fuel",
            TextStyle {
                font: assets.font_vt323.clone(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(88.0),
            ..default()
        }),
    ));
}

fn update_fuel_bar_system(
    mut fuel_bar_sprite_query: Query<&mut Sprite, With<FuelBar>>,
    scores: Res<Scores>,
) {
    let Ok(mut fuel_bar_sprite) = fuel_bar_sprite_query.get_single_mut() else {
        return;
    };
    fuel_bar_sprite.custom_size = Some(Vec2::new(scores.fuel_quantity, 15.0));
}

// Components
#[derive(Component, Debug)]
pub struct FuelBar;
