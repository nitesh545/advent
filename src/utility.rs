use crate::components_and_resources::{HitSoundBulletMeteor, Smoke};
use bevy::audio::Volume;
use bevy::prelude::*;
use std::path::PathBuf;

pub fn spawn_sprite(
    commands: &mut Commands,
    asset_server: AssetServer,
    sprite_asset_path: PathBuf,
    transform: Transform,
) -> Entity {
    commands
        .spawn((
            Sprite::from_image(asset_server.load(sprite_asset_path)),
            transform,
            Smoke {
                duration: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ))
        .id()
}

pub fn spawn_audio(
    commands: &mut Commands,
    asset_server: AssetServer,
    sound_asset_path: PathBuf,
    volume: f32,
    duration: f32,
) -> Entity {
    commands
        .spawn((
            AudioPlayer::new(asset_server.load(sound_asset_path)),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(volume)),
            HitSoundBulletMeteor {
                duration: Timer::from_seconds(duration, TimerMode::Once),
            },
        ))
        .id()
}
