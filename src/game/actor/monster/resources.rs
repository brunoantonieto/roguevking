use bevy::prelude::*;

pub const MONSTER_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct MonsterSpawnTimer {
    pub timer: Timer,
}

impl Default for MonsterSpawnTimer {
    fn default() -> MonsterSpawnTimer {
        MonsterSpawnTimer {
            timer: Timer::from_seconds(MONSTER_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
