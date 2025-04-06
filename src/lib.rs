mod adventui;
mod components_and_resources;
mod enemy;
mod envtools;
mod game;
mod game_plugin;
mod player;
use bevy::prelude::*;

#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    game::run();
}
