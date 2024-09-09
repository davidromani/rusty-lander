use crate::game::Scores;
use crate::spaceship::PlayerAction;
use crate::state::GameState;
use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};
use leafwing_input_manager::prelude::*;

const BIG_THRUST: f32 = 0.75;
const MEDIUM_THRUST: f32 = 0.55;
const SMALL_THRUST: f32 = 0.45;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Landing),
            |mut physics_time: ResMut<Time<Physics>>| {
                physics_time.unpause();
            },
        )
        .add_systems(
            OnEnter(GameState::Paused),
            |mut physics_time: ResMut<Time<Physics>>| {
                physics_time.pause();
            },
        )
        .add_systems(
            Update,
            (
                update_ready_to_land,
                update_grounded,
                movement_system,
                apply_movement_damping,
            )
                .chain()
                .run_if(in_state(GameState::Landing)),
        );
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Debug)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ReadyToLand;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_time_of_impact(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });
        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Updates the [`ReadyToLand`] status for character controllers.
fn update_ready_to_land(
    mut commands: Commands,
    mut query: Query<(Entity, &LinearVelocity), With<CharacterController>>,
) {
    let Ok((entity, linear_velocity)) = query.get_single_mut() else {
        return;
    };
    if linear_velocity.y < 12.5 && linear_velocity.y > -12.5 {
        commands.entity(entity).insert(ReadyToLand);
    } else {
        commands.entity(entity).remove::<ReadyToLand>();
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement_system(
    time: Res<Time>,
    mut scores: ResMut<Scores>,
    mut controllers: Query<(
        &ActionState<PlayerAction>,
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise, you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (action_state, movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
        &mut controllers
    {
        if action_state.pressed(&PlayerAction::LeftThruster) {
            linear_velocity.x += movement_acceleration.0 * delta_time;
            scores.fuel_quantity -= 20.0 * time.delta_seconds();
        }
        if action_state.pressed(&PlayerAction::RightThruster) {
            linear_velocity.x += -movement_acceleration.0 * delta_time;
            scores.fuel_quantity -= 20.0 * time.delta_seconds();
        }
        if !is_grounded {
            if action_state.pressed(&PlayerAction::MainThrusterBig) {
                linear_velocity.y += jump_impulse.0 * BIG_THRUST;
                scores.fuel_quantity -= 100.0 * time.delta_seconds();
            }
            if action_state.pressed(&PlayerAction::MainThrusterMedium) {
                linear_velocity.y += jump_impulse.0 * MEDIUM_THRUST;
                scores.fuel_quantity -= 50.0 * time.delta_seconds();
            }
            if action_state.pressed(&PlayerAction::MainThrusterSmall) {
                linear_velocity.y += jump_impulse.0 * SMALL_THRUST;
                scores.fuel_quantity -= 20.0 * time.delta_seconds();
            }
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
