use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use rand::{thread_rng, Rng};
use crate::view::character::Character;
use crate::view::items::weapons::Weapon;
use crate::view::items::weapons::sword::Sword;

pub struct Enemy {
    pub rect: Rect,
    dx: i32,
    dy: i32,
    pub health: i32,
    pub current_weapon: Box<dyn Weapon>,
    pub is_alive: bool
}

impl Enemy {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = thread_rng();
        Enemy {
            rect: Rect::new(x, y, 50, 50),
            dx: rng.gen_range(-2..=2),
            dy: rng.gen_range(-2..=2),
            health: 10,
            current_weapon: Box::new(Sword::new()),
            is_alive: true,
        }
    }

    pub fn update(&mut self) {
        self.rect.set_x(self.rect.x() + self.dx);
        self.rect.set_y(self.rect.y() + self.dy);

        // Randomize movement occasionally
        let mut rng = thread_rng();
        if rng.gen_range(0..100) < 5 { // 5% chance to change direction
            self.dx = rng.gen_range(-2..=2);
            self.dy = rng.gen_range(-2..=2);
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 255, 0)); // Green color for enemies
        canvas.fill_rect(self.rect).unwrap();
    }

    pub fn check_liveness(&mut self) {
        if self.health <= 0 {
            self.is_alive = false;
        }
    }
}

impl Character for Enemy {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn apply_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn weapon(&self) -> Option<&Box<dyn Weapon>> {
        Some(&self.current_weapon)
    }
}