use bevy::prelude::*;

use super::super::components::WeaponActions;
use super::super::components::Weapon;
use crate::game::actor::player::components::Player;
use crate::game::items::weapons::sword::components::Sword;

pub const SWORD_SPEED: f32 = 5.0;
pub const SWORD_FILE_SIZE: f32 = 818.0;

pub fn sword() {
    return
}

impl WeaponActions for Sword {
    fn spawn_weapon(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
    ) {
        let scale_x = 140.0 / SWORD_FILE_SIZE;
        let scale_y = 140.0 / SWORD_FILE_SIZE;
        parent.spawn((SpriteBundle {
            transform: Transform::from_xyz(-20.0, 0.0, -0.1)
                .with_scale(Vec3::new(scale_x, scale_y, 1.0)),
            texture: asset_server.load("sprites/sword2.png"),
            ..default()
            },
            Weapon { progress: 0.0 }
        ));
    }

    fn weapon_animation(
        player: &mut Player,
        weapon: &mut Weapon,
        weapon_transform: &mut Transform,
        time: Res<Time>,
    ) {
        if weapon.progress > 0.0 {
            // Update the attack progress
            weapon.progress += time.delta_seconds() * SWORD_SPEED;
    
            if weapon.progress >= 1.0 {
                // Reset the attack
                weapon.progress = 0.0;
                weapon_transform.rotation = Quat::IDENTITY; // Reset rotation to default
                player.is_attacking = false;
            } else {
                // Rotate the sword
                let angle = weapon.progress * std::f32::consts::PI / 2.0; // 90 degrees rotation
                weapon_transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}