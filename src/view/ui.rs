use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct UI<'a> {
    font: Font<'a, 'a>,
    kill_count: u32,
    timer: Duration,
}

impl<'a> UI<'a> {
    pub fn new(ttf_context: &'a Sdl2TtfContext) -> UI<'a> {
        let font = ttf_context.load_font("assets/font/King.TTF", 12).unwrap();
        UI {
            font,
            kill_count: 0,
            timer: Duration::new(0, 0),
        }
    }

    pub fn set_kill_count(&mut self, count: u32) {
        self.kill_count = count;
    }

    pub fn update_timer(&mut self, time: Duration) {
        self.timer = time;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        // Render timer
        let timer_text = format!("{:02}:{:02}", self.timer.as_secs() / 60, self.timer.as_secs() % 60);
        let surface = self.font.render(&timer_text).blended(Color::RGBA(0, 0, 0, 0)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let timer_text_position = sdl2::rect::Rect::new(5, 5, surface.width(), surface.height());

        canvas.copy(&texture, None, Some(timer_text_position)).unwrap();

        // Render kill count
        let kill_text = format!("Kills: {}", self.kill_count);
        let surface = self.font.render(&kill_text).blended(Color::RGBA(0, 0, 0, 0)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let kill_text_position = sdl2::rect::Rect::new(5, 20, surface.width(), surface.height());
        canvas.copy(&texture, None, Some(kill_text_position)).unwrap();

    }
}
