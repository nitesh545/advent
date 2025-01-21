use bevy::prelude::*;
use bevy::window::{CursorGrabMode, Monitor, PrimaryWindow, WindowMode};
use rand::{thread_rng, Rng};
use std::time::Duration;
use avian2d::dynamics::integrator::IntegrationSet::Velocity;
use avian2d::prelude::*;
use bevy::audio::{PlaybackMode, Volume};
use bevy::ecs::observer::TriggerTargets;

#[derive(Component)]
struct Player {
    speed: f32,
    acceleration: f32,
    max_speed: f32,
    velocity: Vec3,
    friction: f32,
    fire_delay: Timer,
}

#[derive(Component)]
struct Score {
    score: u32,
}

#[derive(Component)]
struct Accuracy {
    bullets_fired: f32,
    bullets_hit: f32,
    accuracy: f32,
}


#[derive(Component)]
struct Rock {
    health: f32,
}

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct Bullet {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Enemy {
    health: f32,
    direction: Vec3,
    speed: f32,
    enemy_rotation: f32,
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct SpaceStation {
    rotation_speed: f32,
}

#[derive(Component)]
struct Smoke{
    duration: Timer,
}

#[derive(Component)]
struct HitSoundBulletMeteor {
    duration: Timer,
}

#[derive(Component)]
struct BulletFireSound {
    duration: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8, variant: String) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps, variant),
        }
    }

    fn timer_from_fps(fps: u8, variant: String) -> Timer {
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

#[derive(Resource)]
struct EnemySapwnTimer(Timer);

#[derive(Resource)]
struct BulletFadeTimer(Timer);

#[derive(Resource)]
struct PlayerFireAnimationTimer(Timer);

// all basic functionalities like background spawning, changing cursor and setting up camera is
// handled in GamePlugin
struct GamePlugin;
impl GamePlugin{
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2d::default());
    }

    fn setup_space_station(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        commands.spawn(
            (
                Sprite::from_image(asset_server.load("spaceStation2.png")),
                Transform::from_xyz(0.0, 0.0, -1.0),
                SpaceStation{rotation_speed: 0.05},
            ),
        );
    }

    fn rotate_space_station(
        mut q_space_station: Query<(&mut Transform, &SpaceStation), With<SpaceStation>>,
        time: Res<Time>,
    ) {
        let (mut space_station_transform, space_station) = q_space_station.single_mut();
        space_station_transform.rotate_z(time.delta_secs() * space_station.rotation_speed);
    }

    fn setup_background(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn(
            (
                Sprite::from_image(asset_server.load("SpaceBackground1.png")),
                Transform::from_xyz(0.0, 0.0, -5.0).with_scale(Vec3::splat(0.7)),
            )
        );
    }

    fn setup_crosshair(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn((
            Sprite::from_image(asset_server.load("cursor.png")),
            Cursor,
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1)),
        ));
    }

    fn setup_score(mut commands: Commands) {
        let mut score = Score {score: 0};
        let mut accuracy = Accuracy {bullets_fired: 0.0, bullets_hit: 0.0, accuracy: 100.0};

        commands.spawn(
            (
                Text::new(format!("Score: {}", score.score)),
                TextFont {
                    font_size: 45.0,
                    ..default()
                },
                score,
                Node {
                    position_type: PositionType::Relative,
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
            )
        );
        commands.spawn(
            (
                Text::new(format!("Accuracy: {}", accuracy.accuracy)),
                TextFont {
                    font_size: 45.0,
                    ..default()
                },
                accuracy,
                Node {
                    position_type: PositionType::Relative,
                    top: Val::Px(62.0),
                    left: Val::Px(12.0),
                    ..default()
                },
            )
        );
    }

    fn update_score_text(
        mut q_text: Query<(&mut Text, &mut Score), With<Score>>,
    ) {
        let (mut text, mut score) = q_text.single_mut();
        text.0 = format!("Score: {}", score.score);
    }

    fn update_accuracy_text(
        mut q_text: Query<(&mut Text, &mut Accuracy), With<Accuracy>>,
    ) {
        let (mut text, mut accuracy) = q_text.single_mut();
        text.0 = format!("Accuracy: {}", ((accuracy.bullets_hit/accuracy.bullets_fired)*100.0) as i32);
    }

    fn show_score(
        mut q_score: Query<&mut Score>,
    ) {
        let mut score = q_score.single();
        println!("{}", score.score);
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
        cursor_transform.translation.x = cursor_position.x - win_length / 2.0;
        cursor_transform.translation.y = win_height / 2.0 - cursor_position.y;
        cursor_transform.translation.z = 10.0;
    }

    fn setup_music(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn((
            AudioPlayer::new(asset_server.load("space_music.ogg")),
            PlaybackSettings::LOOP,
            ));
    }

    fn despawn_smokes(
        mut q_smoke: Query<(&mut Smoke, Entity, &mut Sprite), With<Smoke>>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        for (mut smoke, entity, mut sprite) in q_smoke.iter_mut() {
            smoke.duration.tick(time.delta());
            let mut remaining = smoke.duration.remaining().as_secs_f32();
            let alpha = ((smoke.duration.duration().as_secs_f32() - remaining)/ 2.0).clamp(0.0, 1.0);
            sprite.color.set_alpha(1.0-alpha);
            if smoke.duration.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn despawn_hit_sounds_bullet_meteor(
        mut q_sound: Query<(&mut HitSoundBulletMeteor, Entity), With<HitSoundBulletMeteor>>,
        mut commands: Commands,
        time: Res<Time>,
    ){
        // let mut v: Vec<Entity> = Vec::new();
        for (mut sound, entity) in q_sound.iter_mut() {
            sound.duration.tick(time.delta());
            // v.push(entity);
            if sound.duration.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
        // println!("{v:?}");
    }
}
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_camera)
            .add_systems(Startup, Self::setup_background)
            .add_systems(Startup, Self::setup_crosshair)
            .add_systems(Startup, Self::setup_score)
            .add_systems(Startup, Self::setup_space_station)
            .add_systems(Startup, Self::setup_music)
            .add_systems(Update, Self::custom_cursor)
            .add_systems(Update, Self::update_score_text)
            .add_systems(Update, Self::rotate_space_station)
            .add_systems(Update, Self::despawn_smokes)
            .add_systems(Update, Self::despawn_hit_sounds_bullet_meteor)
            .add_systems(Update, Self::update_accuracy_text)
        ;
    }
}

struct PlayerPlugin;
impl PlayerPlugin {
    fn setup_player(
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

        commands.spawn((
            // Mesh2d(meshes.add(Circle::new(25.0))),
            // Sprite::from_image(asset_server.load("tower.png")),
            Sprite::from_atlas_image(texture, TextureAtlas {
                layout: texture_atlas_layout,
                index: anim_config.first_sprite_index,
            }),
            // Sprite::from_image(asset_server.load("tower2.png")),
            Transform::from_scale(Vec3::splat(0.5)),
            RigidBody::Kinematic,
            Collider::circle(100.0),
            Sensor,
            // MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
            Player {
                speed: 200.0,
                acceleration: 500.0,
                max_speed: 400.0,
                velocity: Vec3::ZERO,
                friction: 5.0,
                fire_delay: Timer::from_seconds(0.2, TimerMode::Once),
            },
            anim_config,
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

    fn player_rotate(
        mut q_window: Query<&Window, With<PrimaryWindow>>,
        mut q_player: Query<(&mut Transform), With<Player>>,
    ) {
        let win = q_window.single();
        let mut transform = q_player.single_mut();
        let mut position = match win.cursor_position() {
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

    fn fire_bullet(
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
        if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
            let win = q_windows.single();
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
                        // Mesh2d(meshes.add(Circle::new(2.5))),
                        // MeshMaterial2d(materials.add(Color::srgb(0.72, 0.96, 0.97))),
                        Sprite::from_image(asset_server.load("bullet.png")),
                        Transform::from_translation(transform.translation).with_scale(Vec3::splat(0.1)).with_rotation(Quat::from_rotation_z(angle)),
                        Bullet {
                            // speed: 800.0,
                            speed: 0.0,
                            direction: dir,
                        },
                        RigidBody::Dynamic,
                        Collider::circle(5.0),
                        TransformExtrapolation,
                        // ExternalForce::new(avian2d::math::Vector::X * 10000.0),
                        ExternalImpulse::new(avian2d::math::Vector::from((dir.x, dir.y)) * 500.0),
                    ))
                    .id();
                let bullet_fire_entity = commands.spawn((
                    AudioPlayer::new(asset_server.load("fire.ogg")),
                    PlaybackSettings::ONCE,
                    BulletFireSound{duration: Timer::from_seconds(2.0, TimerMode::Once)},
                    )).id();
                let mut accuracy = q_accuracy.single_mut();
                accuracy.bullets_fired += 1.0;
            }
        }
    }

    fn remove_bullet_sound_entities(
        mut s_query: Query<(&mut BulletFireSound, Entity), With<AudioPlayer>>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        // let mut v: Vec<Entity> = Vec::new();
        for (mut fire_sound, entity) in s_query.iter_mut() {
            // v.push(entity);
            fire_sound.duration.tick(time.delta());
            if fire_sound.duration.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        };
        // println!("{v:?}");
    }

    fn move_bullet(mut query: Query<(&mut Transform, &mut Bullet), With<Bullet>>, time: Res<Time>) {
        let time_step = time.delta_secs();
        for (mut transform, mut bullet) in query.iter_mut() {
            transform.translation += bullet.speed * time_step * bullet.direction.normalize();
        }
    }

    fn execute_animations_player(
        time: Res<Time>,
        mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Player>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
    ) {
        for (mut config, mut sprite) in &mut query {
            if mouse_input.just_pressed(MouseButton::Left) {
                config.frame_timer.reset();
            }
            // we track how long the current sprite has been displayed for
            config.frame_timer.tick(time.delta());

            // If it has been displayed for the user-defined amount of time (fps)...
            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index - 1 {
                        // ...and it IS the last frame, then we move back to the first frame and stop.
                        atlas.index = config.first_sprite_index;
                    } else {
                        // ...and it is NOT the last frame, then we move to the next frame...
                        atlas.index += 1;
                        // ...and reset the frame timer to start counting all over again
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
            .add_systems(Update, Self::remove_bullet_sound_entities)
        ;
    }
}

struct EnemyPlugin;

impl EnemyPlugin {
    fn spawn_enemies(
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

            // let texture = asset_server.load("fireball.png");
            // let layout = TextureAtlasLayout::from_grid(UVec2::new(256, 256), 4, 4, None, None);
            // let texture_atlas_layout = texture_atlas_layouts.add(layout);
            // let anim_config = AnimationConfig::new(1, 16, 15, String::from("Repeating"));

            commands.spawn((
                // Mesh2d(meshes.add(Circle::new(25.0))),
                // MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
                // Sprite::from_atlas_image(texture, TextureAtlas {
                //     layout: texture_atlas_layout,
                //     index: anim_config.first_sprite_index,
                // }),
                Sprite::from_image(asset_server.load("rock.png")),
                Transform::from_xyz(
                    rng.gen_range(-1.0 * win_length / 2.0 + 50.0 ..win_length / 2.0 - 50.0),
                    rng.gen_range(-1.0 * win_height / 2.0 + 50.0 ..win_height / 2.0 - 50.0),
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
                ExternalImpulse::new(avian2d::math::Vector::new(enemy_direction.x, enemy_direction.y) * 650000.0),
                Collider::circle(100.0),
                Restitution::new(1.0),
                TransformExtrapolation,
                // Sensor,
                // anim_config,
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

            // rotation logic
            // let dir = enemy_direction.normalize();
            // let angle = dir.y.atan2(dir.x);
            // flames_transform.rotation = Quat::from_rotation_z(angle);
        }
    }

    fn rotate_enemies(
        mut q_enemies: Query<(&mut Transform, &Enemy), With<Enemy>>,
        timer: Res<Time>,
    ) {
        for (mut transform, enemy) in q_enemies.iter_mut() {
            transform.rotate_z(enemy.enemy_rotation * timer.delta_secs());
        }
    }

    fn execute_animations_enemies(
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
            .add_systems(Update, Self::rotate_enemies)
        ;
    }
}

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((DefaultPlugins,
                          // .set(WindowPlugin{primary_window: Some(Window{resizable: false, mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary), ..default()}), ..default()}),
                      PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        // .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(EnemySapwnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
        )))
        .insert_resource(BulletFadeTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_plugins((GamePlugin, PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, setup_bounds)
        .add_systems(Update, debug_inputs)
        .add_systems(Update, collision_reader)
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

// this system is not in use, depricated.
fn collision_bullet_enemy(
    mut q_enemy: Query<(&mut Transform, &mut Enemy, Entity), (With<Enemy>, Without<Bullet>)>,
    mut q_bullet: Query<(&mut Transform, &mut Bullet, Entity), (With<Bullet>, Without<Enemy>)>,
    mut commands: Commands,
    mut q_score: Query<&mut Score>,
) {
    let mut score = q_score.single_mut();
    for (mut transform_e, mut enemy, entity_e) in q_enemy.iter_mut() {
        for (mut transform_b, mut bullet, entity_b) in q_bullet.iter_mut() {
            let right_bound = transform_e.translation.x + 25.0 >= transform_b.translation.x;
            let left_bound = transform_e.translation.x - 25.0 <= transform_b.translation.x;
            let upper_bound = transform_e.translation.y + 25.0 >= transform_b.translation.y;
            let lower_bound = transform_e.translation.y - 25.0 <= transform_b.translation.y;
            if right_bound && left_bound && upper_bound && lower_bound {
                score.score += 1;
                commands.entity(entity_b).despawn_recursive();
                commands.entity(entity_e).despawn_recursive();
            }
        }
    }
}

// this system is not in use, depricated.
fn collision_player_enemy(
    mut q_enemy: Query<(&mut Transform, &mut Enemy), (With<Enemy>, Without<Player>)>,
    mut q_player: Query<(&mut Transform, &mut Player), (With<Player>, Without<Enemy>)>,
) {
    for (mut transform_e, mut enemy) in q_enemy.iter_mut() {
        for (mut transform_p, mut player) in q_player.iter_mut() {
            let right_bound = transform_e.translation.x + 25.0 >= transform_p.translation.x;
            let left_bound = transform_e.translation.x - 25.0 <= transform_p.translation.x;
            let upper_bound = transform_e.translation.y + 25.0 >= transform_p.translation.y;
            let lower_bound = transform_e.translation.y - 25.0 <= transform_p.translation.y;
            if right_bound && left_bound && upper_bound && lower_bound {
                transform_p.translation = Vec3::ZERO;
            }
        }
    }
}

fn collision_reader(
    mut collision_event_reader: EventReader<Collision>,
    q_enemy: Query<(&Transform, Entity), With<Enemy>>,
    q_bullet: Query<Entity, With<Bullet>>,
    q_player: Query<Entity, With<Player>>,
    q_wall: Query<Entity, With<Wall>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_score: Query<&mut Score>,
    mut q_accuracy: Query<&mut Accuracy, With<Accuracy>>,
) {
    let mut score = q_score.single_mut();
    let mut accuracy = q_accuracy.single_mut();
    for Collision(contacts) in collision_event_reader.read() {
        // collision b/w meteor and bullet
        if (q_enemy.contains(contacts.entity1) && q_bullet.contains(contacts.entity2)) ||
            (q_enemy.contains(contacts.entity2) && q_bullet.contains(contacts.entity1)) {
            let ind = q_enemy.iter().position(|(&x, ent)| ent == contacts.entity1 || ent == contacts.entity2).unwrap();
            let (&transform, ent) = q_enemy.iter().collect::<Vec<_>>()[ind];
            commands.entity(contacts.entity2).despawn_recursive();
            commands.entity(contacts.entity1).despawn_recursive();
            let collided_at = contacts.manifolds[0].contacts[0].global_point1(&Position::from_xy(transform.translation.x, transform.translation.y), &Rotation::from(transform.rotation));
            commands.spawn((
                Sprite::from_image(asset_server.load("collision_smoke.png")),
                Transform::from_xyz(collided_at.x, collided_at.y, 0.0).with_scale(Vec3::splat(0.25)),
                Smoke{duration: Timer::from_seconds(2.0, TimerMode::Once)},
                ));
            commands.spawn((
                AudioPlayer::new(asset_server.load("explosion.ogg")),
                PlaybackSettings::ONCE.with_volume(Volume::new(5.0)),
                HitSoundBulletMeteor {duration: Timer::from_seconds(2.0, TimerMode::Once)},
                ));
            score.score += 1;
            accuracy.bullets_hit += 1.0;
        }

        // collision b/w meteor and player
        if (q_enemy.contains(contacts.entity1) && q_player.contains(contacts.entity2)) ||
            (q_enemy.contains(contacts.entity2) && q_player.contains(contacts.entity1)) {
            println!("Game Over!");
        }

        // collision b/w wall and bullet
        if q_wall.contains(contacts.entity2) && q_bullet.contains(contacts.entity1) {
            commands.entity(contacts.entity1).despawn_recursive();
        }
        if q_wall.contains(contacts.entity1) && q_bullet.contains(contacts.entity2) {
            commands.entity(contacts.entity2).despawn_recursive();
        }
    }
}

fn setup_bounds (
    mut commands: Commands,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut win = q_window.single_mut();
    println!("{} {}", win.size().x, win.size().y);
    // let right_mid = win.size().x - 1.0 /2.0 + 25.0;
    let right_mid = 640.0 * 2.0;
    // let left_mid = win.size().x/2.0 - 25.0;
    let left_mid = -640.0 * 2.0;
    // let top_mid = win.size().y - 1.0/2.0 + 25.0;
    let top_mid = 360.0 * 2.0;
    // let bottom_mid = win.size().y / 2.0 - 25.0;
    let bottom_mid = -360.0 * 2.0;
    commands.spawn(
        (
            RigidBody::Kinematic,
            Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
            Transform::from_xyz(right_mid, 0.0, 0.0),
            Wall,
        )
    );
    commands.spawn(
        (
            RigidBody::Kinematic,
            Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
            Transform::from_xyz(left_mid, 0.0, 0.0),
            Wall,
        )
    );
    commands.spawn(
        (
            RigidBody::Kinematic,
            Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
            Transform::from_xyz(0.0, top_mid, 0.0),
            Wall,
        )
    );
    commands.spawn(
        (
            RigidBody::Kinematic,
            Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
            Transform::from_xyz(0.0, bottom_mid, 0.0),
            Wall,
        )
    );
}
