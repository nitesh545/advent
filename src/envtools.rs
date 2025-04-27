//use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components_and_resources::{Bullet, Enemy, Score};

pub fn debug_inputs(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _exit: EventWriter<AppExit>,
) {
    let mut primary_window = q_windows.single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::F1) {
        primary_window.cursor_options.visible = false;
    }
    if keyboard_input.pressed(KeyCode::F2) {
        primary_window.cursor_options.visible = true;
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

#[allow(dead_code, unused_variables)]
pub fn collision_bullet_enemy(
    mut q_enemy: Query<(&mut Transform, &mut Enemy, Entity), (With<Enemy>, Without<Bullet>)>,
    mut q_bullet: Query<(&mut Transform, &mut Bullet, Entity), (With<Bullet>, Without<Enemy>)>,
    mut commands: Commands,
    mut q_score: Query<&mut Score>,
) {
    let mut score = q_score.single_mut().unwrap();
    for (transform_e, _enemy, entity_e) in q_enemy.iter_mut() {
        for (transform_b, _bullet, entity_b) in q_bullet.iter_mut() {
            let right_bound = transform_e.translation.x + 25.0 >= transform_b.translation.x;
            let left_bound = transform_e.translation.x - 25.0 <= transform_b.translation.x;
            let upper_bound = transform_e.translation.y + 25.0 >= transform_b.translation.y;
            let lower_bound = transform_e.translation.y - 25.0 <= transform_b.translation.y;
            if right_bound && left_bound && upper_bound && lower_bound {
                score.score += 1;
                commands.entity(entity_b).despawn();
                commands.entity(entity_e).despawn();
            }
        }
    }
}

// pub fn collision_reader(
//     mut collision_event_reader: EventReader<Collision>,
//     q_enemy: Query<(&Transform, Entity), With<Enemy>>,
//     q_bullet: Query<Entity, With<Bullet>>,
//     q_player: Query<Entity, With<Player>>,
//     q_wall: Query<Entity, With<Wall>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut q_score: Query<&mut Score>,
//     mut q_accuracy: Query<&mut Accuracy, With<Accuracy>>,
// ) {
//     let mut score = q_score.single_mut();
//     let mut accuracy = q_accuracy.single_mut();
//     for Collision(contacts) in collision_event_reader.read() {
//         // collision b/w meteor and bullet
//         if (q_enemy.contains(contacts.entity1) && q_bullet.contains(contacts.entity2))
//             || (q_enemy.contains(contacts.entity2) && q_bullet.contains(contacts.entity1))
//         {
//             let ind = q_enemy
//                 .iter()
//                 .position(|(&x, ent)| ent == contacts.entity1 || ent == contacts.entity2)
//                 .unwrap();
//             let (&transform, ent) = q_enemy.iter().collect::<Vec<_>>()[ind];
//             commands.entity(contacts.entity2).despawn_recursive();
//             commands.entity(contacts.entity1).despawn_recursive();
//             let collided_at = contacts.manifolds[0].contacts[0].global_point1(
//                 &Position::from_xy(transform.translation.x, transform.translation.y),
//                 &Rotation::from(transform.rotation),
//             );
//             commands.spawn((
//                 Sprite::from_image(asset_server.load("collision_smoke1.png")),
//                 Transform::from_xyz(collided_at.x, collided_at.y, 0.0)
//                     .with_scale(Vec3::splat(0.75)),
//                 Smoke {
//                     duration: Timer::from_seconds(2.0, TimerMode::Once),
//                 },
//             ));
//             commands.spawn((
//                 AudioPlayer::new(asset_server.load("explosion.ogg")),
//                 PlaybackSettings::ONCE.with_volume(Volume::new(5.0)),
//                 HitSoundBulletMeteor {
//                     duration: Timer::from_seconds(2.0, TimerMode::Once),
//                 },
//             ));
//             score.score += 1;
//             accuracy.bullets_hit += 1.0;
//         }
//
//         // collision b/w meteor and player
//         if (q_enemy.contains(contacts.entity1) && q_player.contains(contacts.entity2))
//             || (q_enemy.contains(contacts.entity2) && q_player.contains(contacts.entity1))
//         {
//             println!("Game Over!");
//         }
//
//         // collision b/w wall and bullet
//         if q_wall.contains(contacts.entity2) && q_bullet.contains(contacts.entity1) {
//             commands.entity(contacts.entity1).despawn_recursive();
//         }
//         if q_wall.contains(contacts.entity1) && q_bullet.contains(contacts.entity2) {
//             commands.entity(contacts.entity2).despawn_recursive();
//         }
//     }
// }
//
// pub fn setup_bounds(mut commands: Commands, mut q_window: Query<&Window, With<PrimaryWindow>>) {
//     let mut win = q_window.single_mut();
//
//     let right_mid = 640.0 * 2.0;
//     let left_mid = -640.0 * 2.0;
//     let top_mid = 360.0 * 2.0;
//     let bottom_mid = -360.0 * 2.0;
//
//     commands.spawn((
//         RigidBody::Kinematic,
//         Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
//         Transform::from_xyz(right_mid, 0.0, 0.0),
//         Wall,
//     ));
//     commands.spawn((
//         RigidBody::Kinematic,
//         Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
//         Transform::from_xyz(left_mid, 0.0, 0.0),
//         Wall,
//     ));
//     commands.spawn((
//         RigidBody::Kinematic,
//         Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
//         Transform::from_xyz(0.0, top_mid, 0.0),
//         Wall,
//     ));
//     commands.spawn((
//         RigidBody::Kinematic,
//         Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
//         Transform::from_xyz(0.0, bottom_mid, 0.0),
//         Wall,
//     ));
// }
