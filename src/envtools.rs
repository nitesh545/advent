use crate::components_and_resources::{
    Accuracy, Bullet, Enemy, HitSoundBulletMeteor, Player, Score, Smoke, Wall,
};
use avian2d::collision::collider::contact_query::contact;
use avian2d::prelude::*;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

#[allow(dead_code, unused_variables, clippy::type_complexity)]
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

#[allow(clippy::too_many_arguments)]
pub fn collision_reader(
    mut collision_events: EventReader<CollisionStarted>,
    q_colliders: Query<(&Collider, &GlobalTransform)>,
    q_enemy: Query<(&Transform, Entity), With<Enemy>>,
    q_bullet: Query<Entity, With<Bullet>>,
    q_player: Query<Entity, With<Player>>,
    q_wall: Query<Entity, With<Wall>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_score: Query<&mut Score>,
    mut q_accuracy: Query<&mut Accuracy, With<Accuracy>>,
) {
    let mut score = q_score.single_mut().unwrap();
    let mut accuracy = q_accuracy.single_mut().unwrap();
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        println!("collision happened");
        let (collider1, transform1) = match q_colliders.get(*entity1) {
            Ok(k) => k,
            Err(_) => continue,
        };
        let (collider2, transform2) = match q_colliders.get(*entity2) {
            Ok(k) => k,
            Err(_) => continue,
        };

        // collision b/w meteor and bullet
        if (q_enemy.contains(*entity1) && q_bullet.contains(*entity2))
            || (q_enemy.contains(*entity2) && q_bullet.contains(*entity1))
        {
            let ind = q_enemy
                .iter()
                .position(|(&_x, ent)| ent == *entity1 || ent == *entity2)
                .unwrap();
            let (&_transform, _ent) = q_enemy.iter().collect::<Vec<_>>()[ind];
            commands.entity(*entity2).despawn();
            commands.entity(*entity1).despawn();
            //let collided_at = manifolds[0].contacts[0].global_point1(
            //    &Position::from_xy(transform.translation.x, transform.translation.y),
            //    &Rotation::from(transform.rotation),
            //);
            let p1 = transform1.translation().truncate();
            let p2 = transform2.translation().truncate();
            let r1 = transform1.rotation().to_euler(EulerRot::XYZ).2;
            let r2 = transform2.rotation().to_euler(EulerRot::XYZ).2;
            let collided_at = match contact(collider1, p1, r1, collider2, p2, r2, 0.1) {
                Ok(k) => k,
                Err(_) => continue,
            };
            let collided_at = match collided_at {
                Some(k) => k,
                None => continue,
            };
            commands.spawn((
                Sprite::from_image(asset_server.load("collision_smoke1.png")),
                Transform::from_xyz(collided_at.local_point1.x, collided_at.local_point1.y, 0.0)
                    .with_scale(Vec3::splat(0.75)),
                Smoke {
                    duration: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
            commands.spawn((
                AudioPlayer::new(asset_server.load("explosion.ogg")),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(5.0)),
                HitSoundBulletMeteor {
                    duration: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
            score.score += 1;
            accuracy.bullets_hit += 1.0;
        }

        // collision b/w meteor and player
        if (q_enemy.contains(*entity1) && q_player.contains(*entity2))
            || (q_enemy.contains(*entity2) && q_player.contains(*entity1))
        {
            println!("Game Over!");
        }

        // collision b/w wall and bullet
        if q_wall.contains(*entity2) && q_bullet.contains(*entity1) {
            commands.entity(*entity1).despawn();
        }
        if q_wall.contains(*entity1) && q_bullet.contains(*entity2) {
            commands.entity(*entity2).despawn();
        }
    }
}

pub fn setup_bounds(mut commands: Commands, mut q_window: Query<&Window, With<PrimaryWindow>>) {
    let _win = q_window.single_mut().unwrap();

    let right_mid = 640.0 * 2.0;
    let left_mid = -640.0 * 2.0;
    let top_mid = 360.0 * 2.0;
    let bottom_mid = -360.0 * 2.0;

    commands.spawn((
        RigidBody::Kinematic,
        Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
        Transform::from_xyz(right_mid, 0.0, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Kinematic,
        Collider::rectangle(10.0, 720.0 * 2.0 + 25.0),
        Transform::from_xyz(left_mid, 0.0, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Kinematic,
        Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
        Transform::from_xyz(0.0, top_mid, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Kinematic,
        Collider::rectangle(1240.0 * 2.0 + 25.0, 10.0),
        Transform::from_xyz(0.0, bottom_mid, 0.0),
        Wall,
    ));
}
