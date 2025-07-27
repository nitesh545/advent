use bevy::prelude::*;
#[allow(unused_imports)]
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;

use crate::asset_loader::ConfigLoader;
use crate::config::Config;
use crate::{components_and_resources, enemy, envtools, game_plugin, player};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        //.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(components_and_resources::EnemySapwnTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating),
        ))
        //.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .insert_resource(components_and_resources::BulletFadeTimer(
            Timer::from_seconds(1.0, TimerMode::Repeating),
        ))
        .add_plugins((
            game_plugin::GamePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
        ))
        .init_asset::<Config>()
        .init_asset_loader::<ConfigLoader>()
        .add_systems(Startup, envtools::setup_config_file)
        .add_systems(Update, envtools::handle_bullet_wall_collision)
        .add_systems(Update, envtools::handle_player_enemy_collision)
        .add_systems(Update, envtools::handle_bullet_enemy_collision)
        .add_systems(Startup, envtools::setup_bounds)
        .add_systems(Update, envtools::debug_inputs)
        //.add_systems(Update, envtools::collision_reader)
        .run();
}
