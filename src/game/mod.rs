pub mod background;
pub mod monster;
mod player;
pub mod score;
pub mod items;
mod systems;
mod ui;

use background::BackgroundPlugin;
use monster::MonsterPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use items::ItemsPlugin;
use systems::*;
use ui::GameUIPlugin;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            // .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // My Plugins
            .add_plugin(BackgroundPlugin)
            .add_plugin(MonsterPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(ItemsPlugin)
            .add_plugin(GameUIPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::PauseMenu)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
