use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components_and_resources::{Accuracy, AnimationConfig, Bullet, BulletFireSound, Player};

pub struct PlayerPlugin;
impl PlayerPlugin {
    pub fn setup_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("turret2_fire_animation.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(256, 256), 3, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let anim_config = AnimationConfig::new(0, 5, 60, String::from("once"));

        // commands.spawn((
        //     // Mesh2d(meshes.add(Circle::new(25.0))),
        //     // Sprite::from_image(asset_server.load("tower.png")),
        //     Sprite::from_atlas_image(
        //         texture,
        //         TextureAtlas {
        //             layout: texture_atlas_layout,
        //             index: anim_config.first_sprite_index,
        //         },
        //     ),
        //     // Sprite::from_image(asset_server.load("tower2.png")),
        //     Transform::from_scale(Vec3::splat(0.5)),
        //     RigidBody::Kinematic,
        //     Collider::circle(100.0),
        //     Sensor,
        //     // MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
        //     Player {
        //         speed: 200.0,
        //         acceleration: 500.0,
        //         max_speed: 400.0,
        //         velocity: Vec3::ZERO,
        //         friction: 5.0,
        //         fire_delay: Timer::from_seconds(0.2, TimerMode::Once),
        //     },
        //     anim_config,
        // ));

        commands
            .spawn((
                Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: anim_config.first_sprite_index,
                }),
                Transform::from_scale(Vec3::splat(0.5)),
                Player {
                    speed: 200.0,
                    acceleration: 500.0,
                    max_speed: 400.0,
                    velocity: Vec3::ZERO,
                    friction: 5.0,
                    fire_delay: Timer::from_seconds(0.2, TimerMode::Once),
                },
            ));
    }

    pub fn player_movement(
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Transform, &mut Player), With<Player>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
    ) {
        let time_step = time.delta_secs();
        let win = q_window.single().unwrap();
        for (mut transform, mut player) in query.iter_mut() {
            let mut input_direction = Vec3::ZERO;

            // Directional Input
            if keyboard_input.pressed(KeyCode::KeyW)
                && transform.translation.y < win.size().y / 2.0 - 100.0
            {
                // input_direction += *transform.up();
                input_direction += Vec3::Y;
            }
            if keyboard_input.pressed(KeyCode::KeyS)
                && transform.translation.y > win.size().y * -1.0 / 2.0 + 100.0
            {
                // input_direction -= *transform.up();
                input_direction -= Vec3::Y;
            }
            if keyboard_input.pressed(KeyCode::KeyA)
                && transform.translation.x > win.size().x * -1.0 / 2.0 + 100.0
            {
                // input_direction -= *transform.right();
                input_direction -= Vec3::X;
            }
            if keyboard_input.pressed(KeyCode::KeyD)
                && transform.translation.x < win.size().x / 2.0 - 100.0
            {
                // input_direction += *transform.right();
                input_direction += Vec3::X;
            }

            // Normalize input direction
            input_direction = input_direction.normalize_or_zero();

            // Acceleration
            if input_direction.length() > 0.0 {
                let acc = player.acceleration;
                let ve = player.velocity;
                let mx_spd = player.max_speed;
                player.velocity += input_direction * acc * time_step;

                // Clamp velocity to max speed
                if ve.length() > mx_spd {
                    player.velocity = ve.normalize() * mx_spd;
                }
            } else {
                let ve = player.velocity;
                let fr = player.friction;
                // Deceleration (Friction)
                player.velocity -= ve * fr * time_step;

                // Stop if velocity is very small
                if ve.length() < 0.01 {
                    player.velocity = Vec3::ZERO;
                }
            }

            // Apply movement
            transform.translation += player.velocity * time_step;
        }
    }

    pub fn player_rotate(
        q_window: Query<&Window, With<PrimaryWindow>>,
        mut q_player: Query<&mut Transform, With<Player>>,
    ) {
        let win = match q_window.single() {
            Ok(k) => k,
            Err(_e) => return,
        };
        let mut transform = match q_player.single_mut() {
            Ok(k) => k,
            Err(_e) => return,
        };
        let position = match win.cursor_position() {
            Some(k) => k,
            None => return,
        };
        // rotation logic
        // don't touch, ever.
        let win_length = win.size().x;
        let win_height = win.size().y;
        let pos = Vec3::from((
            position.x - win_length / 2.0,
            win_height / 2.0 - position.y,
            0.0,
        ));
        let mut dir = pos - transform.translation;
        dir = dir.normalize();
        let angle = dir.y.atan2(dir.x);
        transform.rotation = Quat::from_rotation_z(angle);
    }

    pub fn fire_bullet(
        mut commands: Commands,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        mut query: Query<&mut Transform, With<Player>>,
        mut q_player: Query<&mut Player, With<Player>>,
        mut q_windows: Query<&Window, With<PrimaryWindow>>,
        mut q_accuracy: Query<&mut Accuracy, With<Accuracy>>,
        time: Res<Time>,
    ) {
        let mut player = q_player.single_mut();
        if keyboard_input.just_pressed(KeyCode::Space)
            || mouse_input.just_pressed(MouseButton::Left)
        {
            let win = q_windows.single().unwrap();
            let mut position = win.cursor_position().unwrap();
            let win_length = win.size().x;
            let win_height = win.size().y;
            for mut transform in query.iter_mut() {
                let pos = Vec3::from((
                    position.x - win_length / 2.0,
                    win_height / 2.0 - position.y,
                    0.0,
                ));
                let mut dir = pos - transform.translation;
                dir = dir.normalize();
                let angle = dir.y.atan2(dir.x);
                let bullet = commands
                    .spawn((
                        Sprite::from_image(asset_server.load("bullet.png")),
                        Transform::from_translation(transform.translation)
                            .with_scale(Vec3::splat(0.2))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        Bullet {
                            speed: 400.0,
                            direction: dir,
                        },
                    ))
                    .id();
                let bullet_fire_entity = commands
                    .spawn((
                        AudioPlayer::new(asset_server.load("fire.ogg")),
                        PlaybackSettings::ONCE,
                        BulletFireSound {
                            duration: Timer::from_seconds(2.0, TimerMode::Once),
                        },
                    ))
                    .id();
                let mut accuracy = q_accuracy.single_mut().unwrap();
                accuracy.bullets_fired += 1.0;
            }
        }
    }

    pub fn remove_bullet_sound_entities(
        mut s_query: Query<(&mut BulletFireSound, Entity), With<AudioPlayer>>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        for (mut fire_sound, entity) in s_query.iter_mut() {
            fire_sound.duration.tick(time.delta());
            if fire_sound.duration.just_finished() {
                commands.entity(entity).despawn();
            }
        }
    }

    pub fn move_bullet(
        mut query: Query<(&mut Transform, &mut Bullet), With<Bullet>>,
        time: Res<Time>,
    ) {
        let time_step = time.delta_secs();
        for (mut transform, bullet) in query.iter_mut() {
            transform.translation += bullet.speed * time_step * bullet.direction.normalize();
        }
    }

    pub fn execute_animations_player(
        time: Res<Time>,
        mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Player>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
    ) {
        for (mut config, mut sprite) in &mut query {
            if mouse_input.just_pressed(MouseButton::Left) {
                config.frame_timer.reset();
            }
            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index - 1 {
                        atlas.index = config.first_sprite_index;
                    } else {
                        atlas.index += 1;
                        config.frame_timer =
                            AnimationConfig::timer_from_fps(config.fps, String::from("once"));
                    }
                }
            }
        }
    }
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_player)
            .add_systems(Update, Self::player_rotate)
            .add_systems(Update, Self::fire_bullet)
            .add_systems(Update, Self::move_bullet)
            .add_systems(Update, Self::execute_animations_player)
            .add_systems(Update, Self::remove_bullet_sound_entities);
    }
}
