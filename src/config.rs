use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub assets: Assets,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub background: PathBuf,
    pub turret: PathBuf,
    pub turret_base: PathBuf,
    pub crosshair: PathBuf,
    pub bgmusic: PathBuf,
    pub bullet: PathBuf,
    pub fire_sound_fx: PathBuf,
    pub meteor: PathBuf,
    pub collision_smoke: PathBuf,
    pub collision_sound: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {}

impl Config {
    pub fn load_config() -> Config {
        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let config: Config = toml::from_str(&config_string).unwrap();
        config
    }
}
