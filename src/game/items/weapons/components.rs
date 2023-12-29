use bevy::prelude::*;

use crate::game::actor::player::components::Player;

#[derive(Component)]
pub struct Weapon {
    pub progress: f32,
}

pub trait WeaponActions {
    fn spawn_weapon(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
    );

    fn weapon_animation(
        player: &mut Player,
        weapon: &mut Weapon,
        weapon_transform: &mut Transform,
        time: Res<Time>,
    );
}