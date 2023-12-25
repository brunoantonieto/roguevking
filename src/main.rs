extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::view::character::player::KeyInputs;
use view::background::Background;
use view::character::player::Player;
use view::character::enemy::Enemy;
use view::character::Character;
use crate::model::collision::*;
use std::time::{Duration, Instant};


mod view;
mod model;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Viking Game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game objects
    let mut player: Player = Player::new(50, 50);
    let mut key_inputs = KeyInputs::new();
    let mut enemies = vec![
        Enemy::new(100, 100),
        Enemy::new(200, 200),
    ];

    let mut last_print_time = Instant::now();
    let print_interval = Duration::new(1, 0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => key_inputs.update(&event),
            }
        }
        let mut collision_occurred = false;
        player.update(&key_inputs);

        for enemy in enemies.iter_mut() {
            enemy.update(player.rect.x(), player.rect.y()); // Update enemy positions
            if enemy.is_alive {
                if let Some(weapon) = player.weapon() {
                    if check_collision(weapon.rect(player.rect.x(), player.rect.y()), enemy.rect()) {
                        enemy.apply_damage(weapon.damage());
                        collision_occurred = true;
                        println!("collision");
                    }
                }
            }

            enemy.check_liveness()
        }

        if collision_occurred && last_print_time.elapsed() >= print_interval {
            println!("Player Health: {}", player.health());
            for (index, enemy) in enemies.iter().enumerate() {
                println!("Enemy {} Health: {}", index + 1, enemy.health());
            }
            last_print_time = Instant::now(); // Reset the timer
        }

        Background::render(&mut canvas);

        player.render(&mut canvas);

        for enemy in &enemies {
        if enemy.is_alive {
            enemy.render(&mut canvas);
        }
        }

        canvas.present();

        // Delay to control frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
