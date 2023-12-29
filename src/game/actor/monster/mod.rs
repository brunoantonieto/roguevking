pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

use super::super::SimulationState;

pub const MONSTER_SIZE: f32 = 130.0; // The monster sprite is 130x130 pixels.
pub const MONSTER_SPEED: f32 = 200.0;
pub const NUMBER_OF_MONSTERS: usize = 4;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<MonsterSpawnTimer>()
            // Startup Systems
            // .add_startup_system(spawn_monsters)
            // Enter State Systems
            .add_system(spawn_monsters.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    monster_movement,
                    update_monster_direction,
                    confine_monster_movement,
                    tick_monster_spawn_timer,
                    spawn_monsters_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_monsters);
    }
}
