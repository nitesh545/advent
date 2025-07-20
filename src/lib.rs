mod adventui;
mod components_and_resources;
mod config;
mod enemy;
mod envtools;
mod game;
mod game_plugin;
mod player;
mod utility;
use bevy::prelude::*;

#[bevy_main]
fn main() {
    run_game();
}

#[unsafe(no_mangle)]
pub fn run_game() {
    game::run();
}
