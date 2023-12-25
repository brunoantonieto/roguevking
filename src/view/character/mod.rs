use sdl2::rect::Rect;
use crate::view::items::weapons::Weapon;

pub mod player;
pub mod enemy;

pub trait Character {
    fn rect(&self) -> Rect;
    fn health(&self) -> i32;
    fn apply_damage(&mut self, amount: i32);
    fn weapon(&self) -> Option<&Box<dyn Weapon>>;
}

