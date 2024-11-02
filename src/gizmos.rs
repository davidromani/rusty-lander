use crate::collider::{PLATFORM_10X_CENTER, PLATFORM_2X_CENTER, PLATFORM_5X_CENTER};
use crate::game::InGameSet;
use crate::spaceship::Player;
use crate::state::GameState;
use bevy::color::palettes::css::YELLOW_GREEN;
use bevy::prelude::*;

const PROXIMITY_RADIUS: f32 = 180.0;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            draw_platform_gizmos_system
                .run_if(in_state(GameState::Landing))
                .in_set(InGameSet::Collisions),
        );
    }
}

// Systems
fn draw_platform_gizmos_system(
    mut gizmos: Gizmos,
    spaceship_query: Query<(&Transform), With<Player>>,
) {
    for (transform) in &spaceship_query {
        gizmos.circle_2d(
            PLATFORM_2X_CENTER,
            PROXIMITY_RADIUS,
            Color::from(YELLOW_GREEN),
        );
        gizmos.circle_2d(
            PLATFORM_5X_CENTER,
            PROXIMITY_RADIUS,
            Color::from(YELLOW_GREEN),
        );
        gizmos.circle_2d(
            PLATFORM_10X_CENTER,
            PROXIMITY_RADIUS,
            Color::from(YELLOW_GREEN),
        );
    }
}
