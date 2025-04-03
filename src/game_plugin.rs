use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components_and_resources::{
    Accuracy, Cursor, HitSoundBulletMeteor, Score, Smoke, SpaceStation,
};

// all basic functionalities like background spawning, changing cursor and setting up camera is
// handled in GamePlugin
pub struct GamePlugin;
impl GamePlugin {
    pub fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2d::default());
    }

    pub fn setup_space_station(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Sprite::from_image(asset_server.load("spaceStation4.png")),
            Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::splat(0.1)),
            SpaceStation {
                rotation_speed: 0.06,
            },
        ));
    }

    pub fn rotate_space_station(
        mut q_space_station: Query<(&mut Transform, &SpaceStation), With<SpaceStation>>,
        time: Res<Time>,
    ) {
        let (mut space_station_transform, space_station) = q_space_station.single_mut();
        space_station_transform.rotate_z(time.delta_secs() * space_station.rotation_speed);
    }

    pub fn setup_background(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn((
            Sprite::from_image(asset_server.load("SpaceBackground1.png")),
            Transform::from_xyz(0.0, 0.0, -5.0).with_scale(Vec3::splat(0.7)),
        ));
    }

    pub fn setup_crosshair(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn((
            Sprite::from_image(asset_server.load("cursor.png")),
            Cursor,
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1)),
        ));
    }

    pub fn setup_score(mut commands: Commands) {
        let score = Score { score: 0 };
        let accuracy = Accuracy {
            bullets_fired: 0.0,
            bullets_hit: 0.0,
            accuracy: 100.0,
        };

        commands.spawn((
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
        ));
        commands.spawn((
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
        ));
    }

    pub fn update_score_text(mut q_text: Query<(&mut Text, &mut Score), With<Score>>) {
        let (mut text, score) = q_text.single_mut();
        text.0 = format!("Score: {}", score.score);
    }

    pub fn update_accuracy_text(mut q_text: Query<(&mut Text, &mut Accuracy), With<Accuracy>>) {
        let (mut text, accuracy) = q_text.single_mut();
        text.0 = format!(
            "Accuracy: {}",
            ((accuracy.bullets_hit / accuracy.bullets_fired) * 100.0) as i32
        );
    }

    #[allow(dead_code)]
    pub fn show_score(q_score: Query<&mut Score>) {
        let score = q_score.single();
        println!("{}", score.score);
    }

    pub fn custom_cursor(
        q_window: Query<&Window, With<PrimaryWindow>>,
        _asset_server: Res<AssetServer>,
        mut q_cursor: Query<&mut Transform, With<Cursor>>,
    ) {
        let win = q_window.single();
        let cursor_position = match win.cursor_position() {
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

    pub fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            AudioPlayer::new(asset_server.load("space_music.ogg")),
            PlaybackSettings::LOOP,
        ));
    }

    pub fn despawn_smokes(
        mut q_smoke: Query<(&mut Smoke, Entity, &mut Sprite), With<Smoke>>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        for (mut smoke, entity, mut sprite) in q_smoke.iter_mut() {
            smoke.duration.tick(time.delta());
            let remaining = smoke.duration.remaining().as_secs_f32();
            let alpha =
                ((smoke.duration.duration().as_secs_f32() - remaining) / 2.0).clamp(0.0, 1.0);
            sprite.color.set_alpha(1.0 - alpha);
            if smoke.duration.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn despawn_hit_sounds_bullet_meteor(
        mut q_sound: Query<(&mut HitSoundBulletMeteor, Entity), With<HitSoundBulletMeteor>>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        for (mut sound, entity) in q_sound.iter_mut() {
            sound.duration.tick(time.delta());
            if sound.duration.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
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
            .add_systems(Update, Self::update_accuracy_text);
    }
}
