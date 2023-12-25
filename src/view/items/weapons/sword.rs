use crate::view::items::weapons::Weapon;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Sword {
    pub height: u32,
    pub width: u32,
    pub padding_x: i32,
    pub padding_y: i32,
    pub damage: i32,
    pub color: Color,
}

impl Sword {
    pub fn new() -> Self {
        Sword {
            height: 50,
            width: 5,
            padding_x: -10,
            padding_y: -35,
            color: Color::RGB(128, 128, 128), // Gray color
            damage: 1,
        }
    }
}

impl Weapon for Sword {
    fn damage(&self) -> i32 {
        self.damage
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas.set_draw_color(self.color);
        let sword_rect = Rect::new(x + self.padding_x, y + self.padding_y, self.width, self.height);
        canvas.fill_rect(sword_rect).expect("Could not render sword");
    }

    fn rect(&self, player_x: i32, player_y: i32) -> Rect {
        Rect::new(player_x + self.padding_x, player_y + self.padding_y, self.width, self.height)
    }
}
