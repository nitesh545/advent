use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::components_and_resources::{AnimationConfig, Enemy, EnemySapwnTimer};

pub struct EnemyPlugin;
#[allow(unused_variables, clippy::too_many_arguments)]
impl EnemyPlugin {
    pub fn spawn_enemies(
        mut commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        mut timer: ResMut<EnemySapwnTimer>,
        time: Res<Time>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = rand::rng();
            let win = q_window.single().unwrap();
            let win_length = win.size().x;
            let win_height = win.size().y;
            let enemy_direction = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            )
            .normalize();
            let enemy_speed = rng.random_range(50.0..200.0);
            let rot = rng.random_range(-4.0..4.0);
            let scale = Vec3::splat(rng.random_range(0.25..0.45));

            commands.spawn((
                Sprite::from_image(asset_server.load("rock.png")),
                Transform::from_xyz(
                    rng.random_range(-win_length / 2.0 + 50.0..win_length / 2.0 - 50.0),
                    rng.random_range(-win_height / 2.0 + 50.0..win_height / 2.0 - 50.0),
                    0.0,
                )
                .with_scale(scale),
                Enemy {
                    health: 100.0,
                    direction: enemy_direction,
                    speed: enemy_speed,
                    //speed: 0.0,
                    enemy_rotation: rot,
                },
                RigidBody::Kinematic,
                Collider::circle(100.0),
                //Sensor,
            ));
        }
    }

    pub fn move_enemies(
        mut query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
        time: Res<Time>,
        q_window: Query<&Window, With<PrimaryWindow>>,
    ) {
        let win = q_window.single().unwrap();
        let time_step = time.delta_secs();
        for (mut transform, mut enemy) in query.iter_mut() {
            if transform.translation.x >= win.size().x / 2.0 - 25.0
                || transform.translation.x <= -win.size().x / 2.0 + 25.0
            {
                enemy.direction.x *= -1.0;
            }
            if transform.translation.y >= win.size().y / 2.0 - 25.0
                || transform.translation.y <= -win.size().y / 2.0 + 25.0
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

    #[allow(dead_code)]
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
