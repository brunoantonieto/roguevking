use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub is_attacking: bool
}
