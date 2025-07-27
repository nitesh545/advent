use bevy_rapier2d::prelude::*;
use std::path::PathBuf;

use crate::components_and_resources::{Accuracy, Bullet, ConfigHandle, Enemy, Player, Score, Wall};
use crate::config::Config;
use crate::utility;
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

//#[allow(clippy::too_many_arguments)]
//pub fn collision_reader(
//    mut collision_events: EventReader<CollisionEvent>,
//    rapier_context: Res<RapierContext>,
//    q_colliders: Query<(&Collider, &GlobalTransform)>,
//    q_enemy: Query<(&Transform, Entity), With<Enemy>>,
//    q_bullet: Query<Entity, With<Bullet>>,
//    q_player: Query<Entity, With<Player>>,
//    q_wall: Query<Entity, With<Wall>>,
//    mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    mut q_score: Query<&mut Score>,
//    mut q_accuracy: Query<&mut Accuracy, With<Accuracy>>,
//) {
//    let mut score = q_score.single_mut().unwrap();
//    let mut accuracy = q_accuracy.single_mut().unwrap();
//    for collision_event in collision_events.read() {
//        match collision_event {
//            CollisionEvent::Started(entity1, entity2, _flags) => {
//                // collision b/w meteor and bullet
//                if (q_enemy.contains(*entity1) && q_bullet.contains(*entity2))
//                    || (q_enemy.contains(*entity2) && q_bullet.contains(*entity1))
//                {
//                    let ind = q_enemy
//                        .iter()
//                        .position(|(&_x, ent)| ent == *entity1 || ent == *entity2)
//                        .unwrap();
//                    let (&transform, _ent) = q_enemy.iter().collect::<Vec<_>>()[ind];
//                    commands.entity(*entity2).despawn();
//                    commands.entity(*entity1).despawn();
//                    //let collided_at = manifolds[0].contacts[0].global_point1(
//                    //    &Position::from_xy(transform.translation.x, transform.translation.y),
//                    //    &Rotation::from(transform.rotation),
//                    //);
//                    let p1 = transform1.translation().truncate();
//                    let p2 = transform2.translation().truncate();
//                    let r1 = transform1.rotation().to_euler(EulerRot::XYZ).2;
//                    let r2 = transform2.rotation().to_euler(EulerRot::XYZ).2;
//                    let collided_at = match contact(collider1, p1, r1, collider2, p2, r2, 0.1) {
//                        Ok(k) => k,
//                        Err(_) => continue,
//                    };
//                    let _collided_at = match collided_at {
//                        Some(k) => k,
//                        None => continue,
//                    };
//
//                    let trans =
//                        Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0)
//                            .with_scale(Vec3::splat(0.5));
//                    let _smoke_sprite = utility::spawn_sprite(
//                        &mut commands,
//                        asset_server.clone(),
//                        PathBuf::from("collision_smoke1.png"),
//                        trans,
//                    );
//
//                    utility::spawn_audio(
//                        &mut commands,
//                        asset_server.clone(),
//                        PathBuf::from("explosion.ogg"),
//                        5.0,
//                        2.0,
//                    );
//                    score.score += 1;
//                    accuracy.bullets_hit += 1.0;
//                }
//
//                // collision b/w meteor and player
//                if (q_enemy.contains(*entity1) && q_player.contains(*entity2))
//                    || (q_enemy.contains(*entity2) && q_player.contains(*entity1))
//                {
//                    let trans = Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.75));
//                    utility::spawn_sprite(
//                        &mut commands,
//                        asset_server.clone(),
//                        PathBuf::from("collision_smoke1.png"),
//                        trans,
//                    );
//                    println!("Game Over!");
//                }
//
//                // collision b/w wall and bullet
//                if q_wall.contains(*entity2) && q_bullet.contains(*entity1) {
//                    commands.entity(*entity1).despawn();
//                }
//                if q_wall.contains(*entity1) && q_bullet.contains(*entity2) {
//                    commands.entity(*entity2).despawn();
//                }
//            }
//            CollisionEvent::Stopped(entity1, entity2, _flags) => {}
//        }
//        //let (collider1, transform1) = match q_colliders.get(*entity1) {
//        //    Ok(k) => k,
//        //    Err(_) => continue,
//        //};
//        //let (collider2, transform2) = match q_colliders.get(*entity2) {
//        //    Ok(k) => k,
//        //    Err(_) => continue,
//        //};
//    }
//}

/// Handles collisions between bullets and walls.
pub fn handle_bullet_wall_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_bullet: Query<Entity, With<Bullet>>,
    q_wall: Query<Entity, With<Wall>>,
) {
    for event in collision_events.read() {
        dbg!("collision event detected");
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let bullet_to_despawn =
                if q_bullet.get(*entity1).is_ok() && q_wall.get(*entity2).is_ok() {
                    Some(*entity1)
                } else if q_bullet.get(*entity2).is_ok() && q_wall.get(*entity1).is_ok() {
                    Some(*entity2)
                } else {
                    None
                };

            if let Some(bullet_entity) = bullet_to_despawn {
                // --- Collision Logic ---
                commands.entity(bullet_entity).despawn();
                dbg!("bullet despawned");
            }
        }
    }
}

/// Handles collisions between the player and enemies.
pub fn handle_player_enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
    q_enemy: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let is_player_enemy_collision = (q_player.get(*entity1).is_ok()
                && q_enemy.get(*entity2).is_ok())
                || (q_player.get(*entity2).is_ok() && q_enemy.get(*entity1).is_ok());

            if is_player_enemy_collision {
                // --- Collision Logic ---

                // Spawn a visual effect at the player's location (or origin)
                let trans = Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.75));
                utility::spawn_sprite(
                    &mut commands,
                    asset_server.clone(),
                    PathBuf::from("collision_smoke1.png"),
                    trans,
                );

                // Here you would typically send a GameOver event or change game state
                // instead of just printing.
                println!("Game Over!");
            }
        }
    }
}

/// Handles collisions between bullets and enemies.
pub fn handle_bullet_enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_bullet: Query<Entity, With<Bullet>>,
    q_enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut q_score: Query<&mut Score>,
    mut q_accuracy: Query<&mut Accuracy>,
    asset_server: Res<AssetServer>,
) {
    let mut score = q_score.single_mut().unwrap();
    let mut accuracy = q_accuracy.single_mut().unwrap();

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // Determine which entity is the bullet and which is the enemy
            let (enemy_entity, enemy_transform, bullet_entity) =
                if let (Ok(bullet_e), Ok((enemy_e, enemy_t))) =
                    (q_bullet.get(*entity1), q_enemy.get(*entity2))
                {
                    (enemy_e, enemy_t, bullet_e)
                } else if let (Ok(bullet_e), Ok((enemy_e, enemy_t))) =
                    (q_bullet.get(*entity2), q_enemy.get(*entity1))
                {
                    (enemy_e, enemy_t, bullet_e)
                } else {
                    continue; // Not a bullet-enemy collision
                };

            // --- Collision Logic ---

            // Despawn entities
            commands.entity(bullet_entity).despawn();
            commands.entity(enemy_entity).despawn();
            dbg!("bullet & enemy despawned");

            // Spawn collision effects
            let effect_transform = Transform::from_translation(enemy_transform.translation)
                .with_scale(Vec3::splat(0.5));
            utility::spawn_sprite(
                &mut commands,
                asset_server.clone(),
                PathBuf::from("collision_smoke1.png"),
                effect_transform,
            );
            dbg!("effect spawned");
            utility::spawn_audio(
                &mut commands,
                asset_server.clone(),
                PathBuf::from("explosion.ogg"),
                5.0,
                2.0,
            );
            dbg!("sound effet for explosion spawned");

            // Update score and accuracy
            score.score += 1;
            accuracy.bullets_hit += 1.0;
            dbg!("Score and Accuracy updated");
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
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 720.0 * 2.0 + 25.0),
        Transform::from_xyz(right_mid, 0.0, 0.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(0.0),
        ColliderMassProperties::Density(2.0),
        //Transform::from_xyz(right_mid, 0.0, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 720.0 * 2.0 + 25.0),
        Transform::from_xyz(left_mid, 0.0, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1240.0 * 2.0 + 25.0, 10.0),
        Transform::from_xyz(0.0, top_mid, 0.0),
        Wall,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1240.0 * 2.0 + 25.0, 10.0),
        Transform::from_xyz(0.0, bottom_mid, 0.0),
        Wall,
    ));
}

pub fn setup_config_file(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_config_file: Handle<Config> = asset_server.load(PathBuf::from("config.toml"));
    commands.insert_resource(ConfigHandle(handle_config_file));
}
