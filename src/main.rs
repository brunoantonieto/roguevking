extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::view::character::player::KeyInputs;
use view::background::Background;
use view::character::player::Player;
use view::character::enemy::Enemy;
use view::character::Character;
use crate::view::ui::UI;
use crate::model::collision::*;
use std::time::{Duration, Instant};

mod view;
mod model;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("rogue vking", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_loader = canvas.texture_creator();

    let mut ui = UI::new(&ttf_context);

    // Game objects
    let mut player: Player = Player::new(50, 50, &texture_loader);
    let mut key_inputs = KeyInputs::new();
    let mut enemies = Vec::new();

    let mut kill_count: u32 = 0;
    let start_time = Instant::now();

    // let elapsed_time = start_time.elapsed();

    let mut last_enemy_spawn_time = Instant::now();
    let enemy_spawn_interval = Duration::new(1, 0);


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
        if last_enemy_spawn_time.elapsed() >= enemy_spawn_interval {
            enemies.push(Enemy::new(rand::random::<i32>() % 800, rand::random::<i32>() % 600, 1));
            last_enemy_spawn_time = Instant::now();
        }
        let mut collision_occurred = false;
        player.update(&key_inputs);

        for enemy in enemies.iter_mut() {
            enemy.update(player.rect.x(), player.rect.y());
            if enemy.is_alive {
                if let Some(weapon) = player.weapon() {
                    if check_collision(weapon.rect(player.rect.x(), player.rect.y()), enemy.rect()) {
                        enemy.apply_damage(weapon.damage());
                        collision_occurred = true;
                        println!("collision");
                        if enemy.health() <= 0 {
                            kill_count += 1;
                        }
                    }
                }
            }

            enemy.check_liveness()
        }

        ui.set_kill_count(kill_count);
        ui.update_timer(start_time.elapsed());

        if collision_occurred && last_print_time.elapsed() >= print_interval {
            println!("Player Health: {}", player.health());
            for (index, enemy) in enemies.iter().enumerate() {
                println!("Enemy {} Health: {}", index + 1, enemy.health());
            }
            last_print_time = Instant::now();
        }

        Background::render(&mut canvas);
        ui.render(&mut canvas);

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
