use crate::game::Scores;
use crate::spaceship::{HorizontalThrusterEffect, Player, PlayerAction, VerticalThrusterEffect};
use crate::state::GameState;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ParticlesThrusterPlugin;

impl Plugin for ParticlesThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(
                Update,
                (
                    add_vertical_thrust_particles_to_spaceship_system,
                    add_left_thrust_particles_to_spaceship_system,
                ),
            )
            .add_systems(
                Update,
                update_vertical_thrust_particles_system.run_if(in_state(GameState::Landing)),
            );
    }
}

// Systems
fn add_vertical_thrust_particles_to_spaceship_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    added_ships: Query<Entity, Added<Player>>,
) {
    for ship_entity in added_ships.iter() {
        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.4).expr();
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
            .with_name("VerticalThrusterEffect")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.0)),
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
                VerticalThrusterEffect,
            ));
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect2).with_z_layer_2d(Some(10.0)),
                    transform: Transform::from_translation(Vec3::new(8.0, -30.0, 0.0)),
                    ..default()
                },
                VerticalThrusterEffect,
            ));
        });
    }
}

fn add_left_thrust_particles_to_spaceship_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    added_ships: Query<Entity, Added<Player>>,
) {
    for ship_entity in added_ships.iter() {
        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.2).expr();
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.8, 0.8, 0.8, 0.9));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.lit(-5.0).expr(),
            base_radius: writer.lit(1.1).expr(),
            top_radius: writer.lit(0.5).expr(),
            dimension: ShapeDimension::Volume,
        };
        let init_vel = SetVelocitySphereModifier {
            speed: writer.lit(100.0).uniform(writer.lit(400.0)).expr(),
            center: writer.lit(Vec3::new(-1.0, 0.0, 0.0)).expr(),
        };
        let effect = effects.add(
            EffectAsset::new(
                vec![8012],
                Spawner::once(10.0.into(), false),
                writer.finish(),
            )
            .with_name("LeftThrusterEffect")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.0)),
                screen_space_size: true,
            }),
        );
        commands.entity(ship_entity).with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
                    transform: Transform::from_translation(Vec3::new(-28.0, 0.0, 0.0)),
                    ..default()
                },
                HorizontalThrusterEffect,
            ));
        });
    }
}

fn update_vertical_thrust_particles_system(
    scores: ResMut<Scores>,
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut exhaust_effect: Query<&mut EffectSpawner, With<VerticalThrusterEffect>>,
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
