use bevy::prelude::*;

pub mod systems;
pub mod components;

use systems::*;

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(sword);
            // .add_system(despawn_sword.in_schedule(OnExit(AppState::Game)));
    }
}
