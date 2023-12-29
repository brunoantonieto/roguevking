use bevy::prelude::*;

#[derive(Component)]
pub struct Monster {
    pub health: i32,
    pub direction: Vec2,
}
