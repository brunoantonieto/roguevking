pub mod components;
mod systems;

use systems::*;

use super::super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // .add_systems(
            //     (monster_hit_player, player_hit_star)
            //         .in_set(OnUpdate(AppState::Game))
            //         .in_set(OnUpdate(SimulationState::Running)),
            // )
            .add_system(attack_input)
            .add_system(attack_animation_system)
            .add_system(player_hit_monster)
            .add_system(
                (player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
