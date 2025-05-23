use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

// use crate::events::GameOver;
use crate::game::actor::monster::components::*;
use crate::game::actor::monster::MONSTER_SIZE;
use crate::game::score::resources::*;
use crate::game::items::star::components::Star;
use crate::game::items::star::STAR_SIZE;
use crate::game::items::weapons::components::Weapon;
use crate::game::items::weapons::components::WeaponActions;
use crate::game::items::weapons::sword::components::Sword;
use crate::game::items::weapons::systems::trigger_attack;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_DEFAULT_HEALTH: i32 = 10;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player2.png"),
            ..default()
        },
        Player { is_attacking: false, health: PLAYER_DEFAULT_HEALTH },
    )).with_children( |parent| {
        <Sword as WeaponActions>::spawn_weapon(parent, &asset_server);
    });
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}


pub fn attack_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut weapon_query: Query<&mut Weapon, With<Weapon>>,
    mut player_query: Query<&mut Player, With<Player>>   
) {
    if keyboard_input.just_pressed(KeyCode::X) {
        if let Ok(mut player) = player_query.get_single_mut() {
            if let Ok(mut weapon) = weapon_query.get_single_mut() {
                trigger_attack(&mut player, &mut weapon)
            }
        }
    }
}

pub fn attack_animation_system(
    time: Res<Time>,
    mut weapon_query: Query<(&mut Weapon, &mut Transform), With<Weapon>>,
    mut player_query: Query<&mut Player, With<Player>>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if let Ok((mut weapon, mut weapon_transform)) = weapon_query.get_single_mut() {
            <Sword as WeaponActions>::weapon_animation(&mut player, &mut weapon, &mut weapon_transform, time);
        }
    }
}

pub fn player_hit_monster(
    // mut commands: Commands,
    player_query: Query<(&Player, &Transform), With<Player>>,
    mut monster_query: Query<(&Transform, &mut Monster), With<Monster>>,
    // asset_server: Res<AssetServer>,
    // audio: Res<Audio>,
    // mut score: ResMut<Score>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        // i will use this later to get the sword damage
        // if let Ok(sword_transform) = sword_query.get_single() {
            for (monster_transform, mut monster) in monster_query.iter_mut() {
                let player_position_with_offset = Vec3::new(
                    player_transform.translation.x - 20.0,
                    player_transform.translation.y,
                    player_transform.translation.z,
                );

                let distance = player_position_with_offset
                    .distance(monster_transform.translation);

                if player.is_attacking && distance < MONSTER_SIZE / 2.0 + PLAYER_SIZE / 2.0 {
                    println!("Player hit monster!");
                    monster.health -= 1;
                    // let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    // audio.play(sound_effect);
                    // commands.entity(monster_entity).despawn();
                }
            }
        // }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}
