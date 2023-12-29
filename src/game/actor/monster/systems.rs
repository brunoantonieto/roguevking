use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

// use crate::monster::components::*;
use super::components::*;
use super::resources::*;
use super::{MONSTER_SIZE, MONSTER_SPEED, NUMBER_OF_MONSTERS};

pub const MONSTER_HEALTH: i32 = 20;

pub fn spawn_monsters(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_MONSTERS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/monster.png"),
                ..default()
            },
            Monster {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                health: MONSTER_HEALTH
            },
        ));
    }
}

pub fn despawn_monsters(
    mut commands: Commands,
    monster_query: Query<(Entity, &mut Monster), With<Monster>>
) {
    for (monster_entity, monster) in monster_query.iter() {
        if monster.health < 0 {
            commands.entity(monster_entity).despawn();
        }
    }
}

pub fn monster_movement(mut monster_query: Query<(&mut Transform, &Monster)>, time: Res<Time>) {
    for (mut transform, monster) in monster_query.iter_mut() {
        let direction = Vec3::new(monster.direction.x, monster.direction.y, 0.0);
        transform.translation += direction * MONSTER_SPEED * time.delta_seconds();
    }
}

pub fn update_monster_direction(
    mut monster_query: Query<(&Transform, &mut Monster)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_monster_size = MONSTER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_monster_size;
    let x_max = window.width() - half_monster_size;
    let y_min = 0.0 + half_monster_size;
    let y_max = window.height() - half_monster_size;

    for (transform, mut monster) in monster_query.iter_mut() {
        // let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            monster.direction.x *= -1.0;
            // direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            monster.direction.y *= -1.0;
            // direction_changed = true;
        }

        // Play SFX
        // if direction_changed {
        //     // Play Sound Effect
        //     let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
        //     let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
        //     // Randomly play one of the two sound effects.
        //     let sound_effect = if random::<f32>() > 0.5 {
        //         sound_effect_1
        //     } else {
        //         sound_effect_2
        //     };
        //     audio.play(sound_effect);
        // }
    }
}

pub fn confine_monster_movement(
    mut monster_query: Query<&mut Transform, With<Monster>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_monster_size = MONSTER_SIZE / 2.0;
    let x_min = 0.0 + half_monster_size;
    let x_max = window.width() - half_monster_size;
    let y_min = 0.0 + half_monster_size;
    let y_max = window.height() - half_monster_size;

    for mut transform in monster_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the monster x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the monster y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_monster_spawn_timer(mut monster_spawn_timer: ResMut<MonsterSpawnTimer>, time: Res<Time>) {
    monster_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_monsters_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    monster_spawn_timer: Res<MonsterSpawnTimer>,
) {
    if monster_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/monster.png"),
                ..default()
            },
            Monster {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                health: MONSTER_HEALTH,
            },
        ));
    }
}
