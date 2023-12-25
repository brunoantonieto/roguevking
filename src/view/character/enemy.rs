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
    pub is_alive: bool,
    pub spawn_rate: u32,
}

impl Enemy {
    pub fn new(x: i32, y: i32, spawn_rate: u32) -> Self {
        let mut rng = thread_rng();
        Enemy {
            rect: Rect::new(x, y, 50, 50),
            dx: rng.gen_range(-2..=2),
            dy: rng.gen_range(-2..=2),
            health: 10,
            current_weapon: Box::new(Sword::new()),
            is_alive: true,
            spawn_rate,
        }
    }

    pub fn update(&mut self, player_x: i32, player_y: i32) {
        self.set_direction_towards_player(player_x, player_y);

        self.rect.set_x(self.rect.x() + self.dx);
        self.rect.set_y(self.rect.y() + self.dy);
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

    pub fn set_direction_towards_player(&mut self, player_x: i32, player_y: i32) {
        let direction_x = player_x - self.rect.x();
        let direction_y = player_y - self.rect.y();
        let magnitude = ((direction_x.pow(2) + direction_y.pow(2)) as f64).sqrt();

        let speed_factor: f64 = 5.0;

        self.dx = (direction_x as f64 / magnitude * speed_factor) as i32; // 1.0 is the speed factor
        self.dy = (direction_y as f64 / magnitude * speed_factor) as i32;
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