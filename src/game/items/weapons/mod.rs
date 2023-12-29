use bevy::prelude::*;

pub mod systems;
pub mod components;

pub mod sword;

use sword::SwordPlugin;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SwordPlugin);
    }
}
