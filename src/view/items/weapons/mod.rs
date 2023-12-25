pub mod sword;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub trait Weapon {
    fn damage(&self) -> i32;
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32);
    fn rect(&self, player_x: i32, player_y: i32) -> Rect;
}