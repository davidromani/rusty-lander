use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

use crate::game::Scores;
use crate::state::GameState;

const BIG_THRUST: f32 = 0.75;
const MEDIUM_THRUST: f32 = 0.55;
const SMALL_THRUST: f32 = 0.45;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(Update, (keyboard_input, gamepad_input).chain());
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
                movement,
                apply_movement_damping,
            )
                .chain()
                .run_if(in_state(GameState::Landing)),
        );
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Jump(Scalar),
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

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scores: Res<Scores>,
) {
    if scores.fuel_quantity >= 0.0 {
        // X-axis
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let horizontal = left as i8 - right as i8;
        let direction = horizontal as Scalar;
        if direction != 0.0 {
            movement_event_writer.send(MovementAction::Move(direction));
        }
        // Y-axis
        if keyboard_input.pressed(KeyCode::Digit2) {
            movement_event_writer.send(MovementAction::Jump(BIG_THRUST as Scalar));
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement_event_writer.send(MovementAction::Jump(MEDIUM_THRUST as Scalar));
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement_event_writer.send(MovementAction::Jump(SMALL_THRUST as Scalar));
        }
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        // X-axis
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        if let Some(x) = axes.get(axis_lx) {
            movement_event_writer.send(MovementAction::Move(x as Scalar));
        }
        // Y-axis
        let big_booster_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::North,
        };
        if buttons.pressed(big_booster_button) {
            movement_event_writer.send(MovementAction::Jump(BIG_THRUST as Scalar));
        }
        let medium_booster_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East,
        };
        if buttons.pressed(medium_booster_button) {
            movement_event_writer.send(MovementAction::Jump(MEDIUM_THRUST as Scalar));
        }
        let small_booster_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };
        if buttons.pressed(small_booster_button) {
            movement_event_writer.send(MovementAction::Jump(SMALL_THRUST as Scalar));
        }
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
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise, you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump(boost) => {
                    if !is_grounded {
                        linear_velocity.y += jump_impulse.0 * boost;
                    }
                }
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
