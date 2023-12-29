mod components;
mod systems;

use bevy::prelude::*;
use systems::*;
use crate::AppState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // Systems
            .add_system(spawn_background.in_schedule(OnEnter(AppState::Game)));
            // Exit State Systems
    }
}
