//use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{components_and_resources, enemy, envtools, game_plugin, player};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(PhysicsPlugins::default())
        //.insert_resource(Gravity(Vec2::NEG_Y * 0.0))
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
        //.add_systems(Startup, envtools::setup_bounds)
        .add_systems(Update, envtools::debug_inputs)
        //.add_systems(Update, envtools::collision_reader)
        .run();
}
