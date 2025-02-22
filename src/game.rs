use avian2d::dynamics::integrator::IntegrationSet::Velocity;
use avian2d::prelude::*;
use bevy::audio::{PlaybackMode, Volume};
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, Monitor, PrimaryWindow, WindowMode};
use rand::{Rng, thread_rng};
use std::time::Duration;

use crate::components_and_resources::{
    Accuracy, AnimationConfig, Bullet, BulletFadeTimer, BulletFireSound, Cursor, Enemy,
    EnemySapwnTimer, HitSoundBulletMeteor, Player, PlayerFireAnimationTimer, Rock, Score, Smoke,
    SpaceStation, Wall,
};
use crate::{components_and_resources, enemy, envtools, game, game_plugin, player};

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        .insert_resource(components_and_resources::EnemySapwnTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating),
        ))
        .insert_resource(components_and_resources::BulletFadeTimer(
            Timer::from_seconds(1.0, TimerMode::Repeating),
        ))
        .add_plugins((
            game_plugin::GamePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
        ))
        .add_systems(Startup, envtools::setup_bounds)
        .add_systems(Update, envtools::debug_inputs)
        .add_systems(Update, envtools::collision_reader)
        .run();
}
