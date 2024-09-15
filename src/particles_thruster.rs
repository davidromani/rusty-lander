use avian2d::math::PI;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::Scores;
use crate::spaceship::{
    LeftHorizontalThrusterEffect, Player, PlayerAction, RightHorizontalThrusterEffect,
    VerticalThrusterEffect,
};
use crate::state::GameState;

pub struct ParticlesThrusterPlugin;

impl Plugin for ParticlesThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(
                Update,
                (
                    add_vertical_thrust_particles_to_spaceship_system,
                    add_left_thrust_particles_to_spaceship_system,
                    add_right_thrust_particles_to_spaceship_system,
                ),
            )
            .add_systems(
                Update,
                (
                    update_vertical_thrust_particles_system.run_if(in_state(GameState::Landing)),
                    update_left_thrust_particles_system.run_if(in_state(GameState::Landing)),
                    update_right_thrust_particles_system.run_if(in_state(GameState::Landing)),
                ),
            );
    }
}

// Systems
fn add_vertical_thrust_particles_to_spaceship_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    spaceships_query: Query<Entity, Added<Player>>,
) {
    for spaceship in spaceships_query.iter() {
        let writer = ExprWriter::new();
        let height_property_value = writer.add_property("height", Value::Scalar((-10.0).into()));
        let lifetime = writer.lit(0.4).expr();
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.2, 0.0, 0.9));
        gradient.add_key(0.75, Vec4::new(1.0, 0.8, 0.0, 0.8));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.prop(height_property_value).expr(),
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
        commands.entity(spaceship).with_children(|parent| {
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
    spaceships_query: Query<Entity, Added<Player>>,
) {
    for spaceship in spaceships_query.iter() {
        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.2).expr();
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.8, 0.8, 0.8, 0.9));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.lit(-8.0).expr(),
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
            .with_name("LeftHorizontalThrusterEffect")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.0)),
                screen_space_size: true,
            }),
        );
        commands.entity(spaceship).with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
                    transform: Transform {
                        translation: Vec3::new(-30.0, -23.0, 0.0),
                        rotation: Quat::from_rotation_z(1.625 * PI),
                        ..default()
                    },
                    ..default()
                },
                LeftHorizontalThrusterEffect,
            ));
        });
    }
}

fn add_right_thrust_particles_to_spaceship_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    spaceships_query: Query<Entity, Added<Player>>,
) {
    for spaceship in spaceships_query.iter() {
        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.2).expr();
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.8, 0.8, 0.8, 0.9));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.lit(-8.0).expr(),
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
            .with_name("RightHorizontalThrusterEffect")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.0)),
                screen_space_size: true,
            }),
        );
        commands.entity(spaceship).with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
                    transform: Transform {
                        translation: Vec3::new(30.0, -23.0, 0.0),
                        rotation: Quat::from_rotation_z(0.375 * PI),
                        ..default()
                    },
                    ..default()
                },
                RightHorizontalThrusterEffect,
            ));
        });
    }
}

fn update_vertical_thrust_particles_system(
    scores: ResMut<Scores>,
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut vertical_thruster_effect_query: Query<&mut EffectSpawner, With<VerticalThrusterEffect>>,
    mut effect_properties_query: Query<&mut EffectProperties>,
) {
    if scores.fuel_quantity > 0.0 {
        for (action_state, children) in player.iter() {
            let mut height = -10.0;
            if action_state.pressed(&PlayerAction::MainThrusterBig)
                || action_state.pressed(&PlayerAction::MainThrusterMedium)
                || action_state.pressed(&PlayerAction::MainThrusterSmall)
            {
                if action_state.pressed(&PlayerAction::MainThrusterMedium) {
                    height = -6.5;
                } else if action_state.pressed(&PlayerAction::MainThrusterSmall) {
                    height = -2.5;
                }
                for &child in children.iter() {
                    if let Ok(effect_property) = effect_properties_query.get_mut(child) {
                        if let Some(_property) = effect_property.get_stored("height") {
                            EffectProperties::set_if_changed(
                                effect_property,
                                "height",
                                height.into(),
                            );
                        }
                    }
                    if let Ok(mut spawner) = vertical_thruster_effect_query.get_mut(child) {
                        spawner.reset();
                    }
                }
            }
        }
    }
}

fn update_left_thrust_particles_system(
    scores: ResMut<Scores>,
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut exhaust_effect: Query<&mut EffectSpawner, With<LeftHorizontalThrusterEffect>>,
) {
    if scores.fuel_quantity > 0.0 {
        for (action_state, children) in player.iter() {
            if action_state.pressed(&PlayerAction::LeftThruster) {
                for &child in children.iter() {
                    if let Ok(mut spawner) = exhaust_effect.get_mut(child) {
                        spawner.reset();
                    }
                }
            }
        }
    }
}

fn update_right_thrust_particles_system(
    scores: ResMut<Scores>,
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut exhaust_effect: Query<&mut EffectSpawner, With<RightHorizontalThrusterEffect>>,
) {
    if scores.fuel_quantity > 0.0 {
        for (action_state, children) in player.iter() {
            if action_state.pressed(&PlayerAction::RightThruster) {
                for &child in children.iter() {
                    if let Ok(mut spawner) = exhaust_effect.get_mut(child) {
                        spawner.reset();
                    }
                }
            }
        }
    }
}
