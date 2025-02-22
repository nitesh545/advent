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

pub struct EnemyPlugin;

impl EnemyPlugin {
    pub fn spawn_enemies(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        mut q_window: Query<&Window, With<PrimaryWindow>>,
        mut timer: ResMut<EnemySapwnTimer>,
        time: Res<Time>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = rand::thread_rng();
            let win = q_window.single();
            let win_length = win.size().x;
            let win_height = win.size().y;
            let enemy_direction =
                Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
            let enemy_speed = rng.gen_range(50.0..200.0);
            let rot = rng.gen_range(-4.0..4.0);
            let scale = Vec3::splat(rng.gen_range(0.25..0.45));

            commands.spawn((
                Sprite::from_image(asset_server.load("rock.png")),
                Transform::from_xyz(
                    rng.gen_range(-1.0 * win_length / 2.0 + 50.0..win_length / 2.0 - 50.0),
                    rng.gen_range(-1.0 * win_height / 2.0 + 50.0..win_height / 2.0 - 50.0),
                    0.0,
                )
                .with_scale(scale),
                Enemy {
                    health: 100.0,
                    direction: enemy_direction,
                    // speed: enemy_speed,
                    speed: 0.0,
                    enemy_rotation: rot,
                },
                RigidBody::Dynamic,
                ExternalImpulse::new(
                    avian2d::math::Vector::new(enemy_direction.x, enemy_direction.y) * 650000.0,
                ),
                Collider::circle(100.0),
                Restitution::new(1.0),
                TransformExtrapolation,
            ));
        }
    }

    pub fn move_enemies(
        mut query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
        time: Res<Time>,
        mut q_window: Query<&Window, With<PrimaryWindow>>,
    ) {
        let win = q_window.single();
        let time_step = time.delta_secs();
        for (mut transform, mut enemy) in query.iter_mut() {
            if transform.translation.x >= win.size().x / 2.0 - 25.0
                || transform.translation.x <= win.size().x * -1.0 / 2.0 + 25.0
            {
                enemy.direction.x *= -1.0;
            }
            if transform.translation.y >= win.size().y / 2.0 - 25.0
                || transform.translation.y <= win.size().y * -1.0 / 2.0 + 25.0
            {
                enemy.direction.y *= -1.0;
            }
            let enemy_direction = enemy.direction;
            let enemy_speed = enemy.speed;
            transform.translation += enemy_direction * enemy_speed * time_step;
        }
    }

    pub fn rotate_enemies(
        mut q_enemies: Query<(&mut Transform, &Enemy), With<Enemy>>,
        timer: Res<Time>,
    ) {
        for (mut transform, enemy) in q_enemies.iter_mut() {
            transform.rotate_z(enemy.enemy_rotation * timer.delta_secs());
        }
    }

    pub fn execute_animations_enemies(
        time: Res<Time>,
        mut q_enemy: Query<(&mut AnimationConfig, &mut Sprite), With<Enemy>>,
    ) {
        for (mut config, mut sprite) in q_enemy.iter_mut() {
            config.frame_timer.tick(time.delta());
            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index - 1 {
                        atlas.index = config.first_sprite_index;
                    } else {
                        atlas.index += 1;
                        config.frame_timer =
                            AnimationConfig::timer_from_fps(config.fps, String::from("Repeating"));
                    }
                }
            }
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::spawn_enemies)
            .add_systems(Update, Self::move_enemies)
            .add_systems(Update, Self::rotate_enemies);
    }
}
