use bevy::prelude::*;

pub mod player;
pub mod monster;

use player::PlayerPlugin;
use monster::MonsterPlugin;


pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}

