use crate::game::Scores;
use crate::spaceship::{ExhaustEffect, Player, PlayerAction};
use crate::state::GameState;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ParticlesThrusterPlugin;

impl Plugin for ParticlesThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Update, add_thrust_particles_to_spaceship_system)
            .add_systems(
                Update,
                update_thrust_particles_system.run_if(in_state(GameState::Landing)),
            );
    }
}

// Systems
fn add_thrust_particles_to_spaceship_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    added_ships: Query<Entity, Added<Player>>,
) {
    for ship_entity in added_ships.iter() {
        // for Ship exhaust, we store a particle effects on the player
        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.4).expr();
        // gradient for particle color evolution
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.2, 0.0, 0.9));
        gradient.add_key(0.75, Vec4::new(1.0, 0.8, 0.0, 0.8));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.lit(-10.0).expr(),
            base_radius: writer.lit(2.3).expr(),
            top_radius: writer.lit(1.0).expr(),
            dimension: ShapeDimension::Volume,
        };
        let init_vel = SetVelocitySphereModifier {
            speed: writer.lit(100.0).uniform(writer.lit(400.0)).expr(),
            center: writer.lit(Vec3::new(0.0, 1.0, 0.0)).expr(),
        };
        let effect = effects.add(
            EffectAsset::new(
                vec![16024],
                Spawner::once(10.0.into(), false),
                writer.finish(),
            )
            .with_name("Exhaust")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.)),
                screen_space_size: true,
            }),
        );
        let effect2 = effect.clone();
        commands.entity(ship_entity).with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
                    transform: Transform::from_translation(Vec3::new(-8.0, -30.0, 0.0)),
                    ..default()
                },
                ExhaustEffect,
            ));
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect2).with_z_layer_2d(Some(10.0)),
                    transform: Transform::from_translation(Vec3::new(8.0, -30.0, 0.0)),
                    ..default()
                },
                ExhaustEffect,
            ));
        });
    }
}

// Trigger a new particle spawning whenever the Ship Impulse is non-0
fn update_thrust_particles_system(
    scores: ResMut<Scores>,
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut exhaust_effect: Query<&mut EffectSpawner, With<ExhaustEffect>>,
) {
    if scores.fuel_quantity > 0.0 {
        for (action_state, children) in player.iter() {
            if action_state.pressed(&PlayerAction::MainThrusterBig)
                || action_state.pressed(&PlayerAction::MainThrusterMedium)
                || action_state.pressed(&PlayerAction::MainThrusterSmall)
            {
                for &child in children.iter() {
                    if let Ok(mut spawner) = exhaust_effect.get_mut(child) {
                        spawner.reset();
                    }
                }
            }
        }
    }
}
