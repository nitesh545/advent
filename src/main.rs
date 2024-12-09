use std::time::Duration;
use bevy::ecs::bundle::DynamicBundle;
use bevy::input::InputSystem;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use rand::Rng;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
    acceleration: f32,
    max_speed: f32,
    velocity: Vec3,
    friction: f32,
}

#[derive(Component)]
struct Rock {
    health: f32,
}

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct Bullet{
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Enemy{
    health: f32,
    direction: Vec3,
    speed: f32,
}

#[derive(Resource)]
struct EnemySapwnTimer(Timer);

#[derive(Resource)]
struct BulletFadeTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EnemySapwnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(BulletFadeTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, debug_inputs)
        .add_systems(Update, player_movement)
        .add_systems(Update, fire_bullet)
        .add_systems(Update, move_bullet)
        .add_systems(Update, custom_cursor)
        .add_systems(Update, move_enemies)
        .add_systems(Update, collision_bullet_enemy)
        .run();
}

fn debug_inputs(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    let mut primary_window = q_windows.single_mut();
    if keyboard_input.pressed(KeyCode::F1) {
        primary_window.cursor_options.visible = false;
    }
    if keyboard_input.pressed(KeyCode::F2) {
        primary_window.cursor_options.visible = true;
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite::from_image(asset_server.load("cursor.png")), Cursor, Transform::from_xyz(0.0,0.0,0.0).with_scale(Vec3::splat(0.1))));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
        Player{
            speed: 200.0,
            acceleration: 500.0,
            max_speed: 400.0,
            velocity: Vec3::ZERO,
            friction: 5.0,
        }
        ));
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let time_step = time.delta_secs();
    let win = q_window.single();
    for (mut transform, mut player) in query.iter_mut() {
        let mut input_direction = Vec3::ZERO;

        // Directional Input
        if keyboard_input.pressed(KeyCode::KeyW) && transform.translation.y < win.size().y/2.0 - 100.0 {
            input_direction += *transform.up();
        }
        if keyboard_input.pressed(KeyCode::KeyS) && transform.translation.y > win.size().y * -1.0/2.0 + 100.0 {
            input_direction -= *transform.up();
        }
        if keyboard_input.pressed(KeyCode::KeyA) && transform.translation.x > win.size().x * -1.0/2.0 + 100.0 {
            input_direction -= *transform.right();
        }
        if keyboard_input.pressed(KeyCode::KeyD) && transform.translation.x < win.size().x/2.0 -100.0 {
            input_direction += *transform.right();
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
            if ve.length() < 0.01{
                player.velocity = Vec3::ZERO;
            }
        }

        // Apply movement
        transform.translation += player.velocity * time_step;
    }
}

fn fire_bullet(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        let win =  q_windows.single();
        let mut position = win.cursor_position().unwrap();
        let win_length = win.size().x;
        let win_height = win.size().y;
        for mut transform in query.iter_mut() {
            let pos = Vec3::from((position.x - win_length/2.0, win_height/2.0 - position.y, 0.0));
            let mut dir = pos - transform.translation;
            let bullet = commands.spawn(
                (
                    Mesh2d(meshes.add(Circle::new(2.5))),
                    MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
                    Transform::from_translation(transform.translation),
                    Bullet{speed: 800.0, direction: dir},
                )
            ).id();
        }
    }
}

fn move_bullet(
    mut query: Query<(&mut Transform, &mut Bullet), With<Bullet>>,
    time: Res<Time>,
) {
    let time_step = time.delta_secs();
    for (mut transform, mut bullet) in query.iter_mut() {
        transform.translation += bullet.speed * time_step * bullet.direction.normalize();
    }
}

fn custom_cursor(
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
) {
    let win = q_window.single();
    let mut cursor_position = match win.cursor_position() {
        Some(k) => k,
        None => return,
    };
    let win_length = win.size().x;
    let win_height = win.size().y;
    let mut cursor_transform = q_cursor.single_mut();
    cursor_transform.translation.x = cursor_position.x - win_length/2.0;
    cursor_transform.translation.y = win_height/2.0 - cursor_position.y;
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut timer: ResMut<EnemySapwnTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished(){
        let mut rng = rand::thread_rng();
        let win = q_window.single();
        let win_length = win.size().x;
        let win_height = win.size().y;
        let enemy_direction = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
        let enemy_speed = rng.gen_range(50.0..200.0);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
            Transform::from_xyz(rng.gen_range(-1.0*win_length/2.0..win_length/2.0), rng.gen_range(-1.0*win_height/2.0..win_height/2.0), 0.0),
            Enemy{health: 100.0, direction: enemy_direction, speed: enemy_speed},
        ));
    }
}

fn move_enemies(
    mut query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
    time: Res<Time>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let win = q_window.single();
    let time_step = time.delta_secs();
    for (mut transform, mut enemy) in query.iter_mut() {
        if transform.translation.x >= win.size().x/2.0 - 25.0 || transform.translation.x <= win.size().x * -1.0/2.0 + 25.0 {
            enemy.direction.x *= -1.0;
        }
        if transform.translation.y >= win.size().y/2.0 - 25.0 || transform.translation.y <= win.size().y * -1.0/2.0 + 25.0 {
            enemy.direction.y *= -1.0;
        }
        let enemy_direction = enemy.direction;
        let enemy_speed = enemy.speed;
        transform.translation += enemy_direction * enemy_speed * time_step;
    }
}

fn collision_bullet_enemy(
    mut q_enemy: Query<(&mut Transform, &mut Enemy, Entity), (With<Enemy>, Without<Bullet>)>,
    mut q_bullet: Query<(&mut Transform, &mut Bullet, Entity), (With<Bullet>, Without<Enemy>)>,
    mut commands: Commands,
) {
    for (mut transform_e, mut enemy, entity_e) in q_enemy.iter_mut() {
        for (mut transform_b, mut bullet, entity_b) in q_bullet.iter_mut() {
            let right_bound = transform_e.translation.x +25.0 >= transform_b.translation.x;
            let left_bound = transform_e.translation.x - 25.0 <= transform_b.translation.x;
            let upper_bound= transform_e.translation.y + 25.0 >= transform_b.translation.y;
            let lower_bound= transform_e.translation.y - 25.0 <= transform_b.translation.y;
            if right_bound && left_bound && upper_bound && lower_bound {
                commands.entity(entity_b).despawn_recursive();
                commands.entity(entity_e).despawn_recursive();
            }
        }
    }
}