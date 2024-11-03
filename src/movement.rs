use crate::asset_loader::MusicAssets;
use crate::audio::MusicBeginSoundEffect;
use crate::game::{InGameSet, OutOfFuelEvent, Resettable, Scores};
use crate::spaceship::PlayerAction;
use crate::state::{GameState, TenSecondsTimer};
use avian2d::{math::*, prelude::*};
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const BIG_THRUST: f32 = 0.75;
const MEDIUM_THRUST: f32 = 0.55;
const SMALL_THRUST: f32 = 0.45;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Landing),
            |mut physics_time: ResMut<Time<Physics>>,
             mut commands: Commands,
             music_assets: Res<MusicAssets>| {
                physics_time.unpause();
                commands.spawn((
                    Resettable,
                    MusicBeginSoundEffect,
                    AudioBundle {
                        source: music_assets.music_begin.clone(),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            volume: Volume::new(0.5),
                            ..default()
                        },
                    },
                ));
                commands
                    .insert_resource(TenSecondsTimer(Timer::from_seconds(10.0, TimerMode::Once)));
            },
        )
        .add_systems(
            OnEnter(GameState::Paused),
            |mut physics_time: ResMut<Time<Physics>>| {
                physics_time.pause();
            },
        )
        .add_systems(
            FixedUpdate,
            (
                update_ready_to_land_system,
                movement_system,
                apply_movement_damping_system,
            )
                .chain()
                .run_if(in_state(GameState::Landing))
                .in_set(InGameSet::Physics),
        );
    }
}

#[derive(Component, Debug)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ReadyToLand;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct JumpImpulse(Scalar);

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar, jump_impulse: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let caster_shape = collider.clone();

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse);
        self
    }
}

fn update_ready_to_land_system(
    mut commands: Commands,
    mut query: Query<(Entity, &LinearVelocity), With<CharacterController>>,
) {
    let Ok((entity, linear_velocity)) = query.get_single_mut() else {
        return;
    };
    if linear_velocity.y < 1.0 && linear_velocity.y > -35.0 {
        commands.entity(entity).insert(ReadyToLand);
    } else {
        commands.entity(entity).remove::<ReadyToLand>();
    }
}

fn movement_system(
    time: Res<Time>,
    mut out_of_fuel_events: EventWriter<OutOfFuelEvent>,
    mut scores: ResMut<Scores>,
    mut controllers: Query<(
        &ActionState<PlayerAction>,
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
    )>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (action_state, movement_acceleration, jump_impulse, mut linear_velocity) in &mut controllers
    {
        if scores.fuel_quantity > 0.0 {
            if action_state.pressed(&PlayerAction::LeftThruster) {
                linear_velocity.x += movement_acceleration.0 * delta_time;
                scores.fuel_quantity -= 20.0 * time.delta_seconds();
            }
            if action_state.pressed(&PlayerAction::RightThruster) {
                linear_velocity.x += -movement_acceleration.0 * delta_time;
                scores.fuel_quantity -= 20.0 * time.delta_seconds();
            }
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
            if scores.fuel_quantity < 0.0 {
                out_of_fuel_events.send(OutOfFuelEvent {});
            }
        }
    }
}

fn apply_movement_damping_system(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
