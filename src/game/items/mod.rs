use bevy::prelude::*;
use self::{star::StarPlugin, weapons::WeaponsPlugin};

pub mod star;
pub mod weapons;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(StarPlugin)
            .add_plugin(WeaponsPlugin);
    }
}
