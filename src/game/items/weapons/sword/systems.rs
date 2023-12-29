use bevy::prelude::*;

// use super::super::components::Weapon;
use crate::game::player::components::Player;
use crate::game::items::weapons::sword::components::Sword;

pub const SWORD_SPEED: f32 = 5.0;
pub const SWORD_FILE_SIZE: f32 = 818.0;

pub fn sword() {
    return
}

pub fn spawn_sword(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    let scale_x = 140.0 / SWORD_FILE_SIZE;
    let scale_y = 140.0 / SWORD_FILE_SIZE;
    parent.spawn((SpriteBundle {
        transform: Transform::from_xyz(-20.0, 0.0, -0.1)
            .with_scale(Vec3::new(scale_x, scale_y, 1.0)),
        texture: asset_server.load("sprites/sword2.png"),
        ..default()
        },
        Sword { progress: 0.0 }
    ));
}

pub fn attack_animation(
    player: &mut Player,
    // mut weapon: &mut Weapon
    sword: &mut Sword,
    sword_transform: &mut Transform,
    time: Res<Time>,
) {
    if sword.progress > 0.0 {
        // Update the attack progress
        sword.progress += time.delta_seconds() * SWORD_SPEED;

        if sword.progress >= 1.0 {
            // Reset the attack
            sword.progress = 0.0;
            sword_transform.rotation = Quat::IDENTITY; // Reset rotation to default
            player.is_attacking = false;
        } else {
            // Rotate the sword
            let angle = sword.progress * std::f32::consts::PI / 2.0; // 90 degrees rotation
            sword_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
