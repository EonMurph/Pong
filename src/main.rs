mod components;

use components::circle::Circle;
use components::game::Game;
use components::paddle::{self, Paddle};

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::Color;

fn main() {
    // Create the sdl context
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create the window, canvas, and event monitor
    let dim = 800;
    let window = video_subsystem.window("Pong", dim, dim).build().unwrap();
    let mut canvas = window
        .clone()
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialise circle
    let mut circle = Circle {
        x: 30,
        y: 100,
        x_vel: 8,
        y_vel: 6,
        r: 12,
        color: Color::WHITE,
    };
    let mut paddle1 = Paddle::new(10, 200);
    let mut paddle2 = Paddle::new((window.size().0 - 30) as i32, 200);

    'running: loop {
        // Reset the canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
         
        let keyboard_state = event_pump.keyboard_state();
        // Process movement keys
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            paddle1.move_paddle(-paddle1.vel);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            paddle2.move_paddle(-paddle2.vel);
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            paddle1.move_paddle(paddle1.vel);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            paddle2.move_paddle(paddle2.vel);
        }

        // Main game code
        circle.check_colliding(&window);
        circle.update();
        circle.draw(&mut canvas);
        paddle1.draw(&mut canvas);
        paddle2.draw(&mut canvas);

        canvas.present();
    }
}
