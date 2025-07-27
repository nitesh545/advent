use bevy::prelude::*;
use std::time::Duration;

use crate::config::Config;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Score {
    pub score: u32,
}

#[derive(Component)]
pub struct SpaceStation {
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct HitSoundBulletMeteor {
    pub duration: Timer,
}

#[derive(Component)]
pub struct Smoke {
    pub duration: Timer,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub velocity: Vec3,
    pub friction: f32,
    pub fire_delay: Timer,
}

#[derive(Component)]
pub struct Accuracy {
    pub bullets_fired: f32,
    pub bullets_hit: f32,
    pub accuracy: f32,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, variant: String) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps, variant),
        }
    }

    pub fn timer_from_fps(fps: u8, variant: String) -> Timer {
        if variant == "once" {
            Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
        } else {
            Timer::new(
                Duration::from_secs_f32(1.0 / (fps as f32)),
                TimerMode::Repeating,
            )
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vec3,
}

#[derive(Component)]
pub struct BulletFireSound {
    pub duration: Timer,
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct BulletFadeTimer(pub Timer);

#[allow(dead_code)]
#[derive(Resource)]
pub struct PlayerFireAnimationTimer(pub Timer);

#[allow(dead_code)]
#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub direction: Vec3,
    pub speed: f32,
    pub enemy_rotation: f32,
}

#[derive(Resource)]
pub struct EnemySapwnTimer(pub Timer);

#[derive(Resource)]
pub struct ConfigHandle(pub Handle<Config>);
