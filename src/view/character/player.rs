use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use crate::view::character::Character;
use crate::view::items::weapons::*;
use crate::view::items::weapons::sword::Sword;

pub struct Player {
    pub rect: Rect,
    speed: i32,
    pub health: i32,
    pub current_weapon: Box<dyn Weapon>,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            rect: Rect::new(x, y, 25, 25),
            speed: 10,
            health: 10,
            current_weapon: Box::new(Sword::new()),
        }
    }

    pub fn update(&mut self, key_inputs: &KeyInputs) {
        if key_inputs.up {
            self.rect.set_y(self.rect.y() - self.speed);
        }
        if key_inputs.down {
            self.rect.set_y(self.rect.y() + self.speed);
        }
        if key_inputs.left {
            self.rect.set_x(self.rect.x() - self.speed);
        }
        if key_inputs.right {
            self.rect.set_x(self.rect.x() + self.speed);
        }

    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(self.rect).unwrap();
        
        if let Some(weapon) = self.weapon() {
            weapon.render(canvas, self.rect.x(), self.rect.y());
        }
    }
}

pub struct KeyInputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl KeyInputs {
    pub fn new() -> KeyInputs {
        KeyInputs {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn update(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => self.up = true,
            Event::KeyUp { keycode: Some(Keycode::Up), .. } => self.up = false,
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => self.down = true,
            Event::KeyUp { keycode: Some(Keycode::Down), .. } => self.down = false,
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => self.left = true,
            Event::KeyUp { keycode: Some(Keycode::Left), .. } => self.left = false,
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => self.right = true,
            Event::KeyUp { keycode: Some(Keycode::Right), .. } => self.right = false,
            _ => {}
        }
    }
}

impl Character for Player {
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