use crate::collider::{PLATFORM_10X_CENTER, PLATFORM_2X_CENTER, PLATFORM_5X_CENTER};
use crate::game::InGameSet;
use crate::spaceship::Player;
use crate::state::GameState;
use bevy::color::palettes::css::{LIGHT_SALMON, YELLOW_GREEN};
use bevy::prelude::*;

pub const PROXIMITY_RADIUS: f32 = 170.0;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            draw_platform_gizmos_system
                .run_if(in_state(GameState::Landing))
                .in_set(InGameSet::Physics),
        );
    }
}

// Systems
fn draw_platform_gizmos_system(
    mut gizmos: Gizmos,
    spaceship_query: Query<&Transform, With<Player>>,
) {
    let mut color: Color;
    for transform in &spaceship_query {
        if transform.translation.distance(PLATFORM_2X_CENTER) < PROXIMITY_RADIUS {
            color = Color::from(LIGHT_SALMON);
        } else {
            color = Color::from(YELLOW_GREEN);
        }
        gizmos.circle_2d(PLATFORM_2X_CENTER.xy(), PROXIMITY_RADIUS, color);
        if transform.translation.distance(PLATFORM_5X_CENTER) < PROXIMITY_RADIUS {
            color = Color::from(LIGHT_SALMON);
        } else {
            color = Color::from(YELLOW_GREEN);
        }
        gizmos.circle_2d(PLATFORM_5X_CENTER.xy(), PROXIMITY_RADIUS, color);
        if transform.translation.distance(PLATFORM_10X_CENTER) < PROXIMITY_RADIUS {
            color = Color::from(LIGHT_SALMON);
        } else {
            color = Color::from(YELLOW_GREEN);
        }
        gizmos.circle_2d(PLATFORM_10X_CENTER.xy(), PROXIMITY_RADIUS, color);
    }
}
