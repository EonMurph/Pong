mod components;

use std::path::Path;
use std::time::SystemTime;

use components::ball::Ball;
use components::game::Game;
use components::paddle::Paddle;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;

fn main() {
    // Create the sdl and ttf context
    let sdl_context: sdl2::Sdl = sdl2::init().expect("Couldn't initialise the sdl context.");
    let sdl_ttf_context: Sdl2TtfContext =
        sdl2::ttf::init().expect("Couldn't initialise the sdl ttf context.");
    let video_subsystem: sdl2::VideoSubsystem = sdl_context
        .video()
        .expect("Couldn't initialise the VideoSubsystem.");

    // Create the window, canvas, and event monitor
    let dim: u32 = 800;
    let window: sdl2::video::Window = video_subsystem
        .window("Pong", dim, dim)
        .build()
        .expect("Couldn't initialise the window.");
    let window_size: (i32, i32) = (window.size().0 as i32, window.size().1 as i32);
    let mut event_pump: sdl2::EventPump = sdl_context
        .event_pump()
        .expect("Couldn't initialise the EventPump.");

    // Load the font path
    let font_path: &Path = Path::new("res/PixelifySans-Bold.ttf");

    // Initialise the ball, paddles, and game structs
    let mut ball: Ball = Ball::new(30, 100, 12);
    let mut paddle1: Paddle = Paddle::new(10, 200);
    let mut paddle2: Paddle = Paddle::new((window.size().0 - 30) as i32, 200);
    let mut game: Game = Game::new(
        5,
        window
            .clone()
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Couldn't initialise the canvas."),
        sdl_ttf_context
            .load_font(font_path, 40)
            .expect("Couldn't initialise the font."),
    );

    'running: loop {
        let loop_start_time: SystemTime = SystemTime::now();
        // Reset the canvas
        game.canvas.set_draw_color(Color::BLACK);
        game.canvas.clear();

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

        // Main game code
        if !game.over {
            // Handle Paddle Movement
            {
                let window_height: i32 = window.size().1 as i32;
                let keyboard_state: sdl2::keyboard::KeyboardState<'_> = event_pump.keyboard_state();
                // Process movement keys
                if keyboard_state.is_scancode_pressed(Scancode::W) {
                    paddle1.move_paddle(-paddle1.vel, window_height);
                }
                if keyboard_state.is_scancode_pressed(Scancode::Up) {
                    paddle2.move_paddle(-paddle2.vel, window_height);
                }
                if keyboard_state.is_scancode_pressed(Scancode::E) {
                    paddle1.move_paddle(paddle1.vel, window_height);
                }
                if keyboard_state.is_scancode_pressed(Scancode::Down) {
                    paddle2.move_paddle(paddle2.vel, window_height);
                }
            }

            // Handle ball movement
            ball.check_colliding([&paddle1, &paddle2], &mut game, &window);
            ball.update_movement();
        }

        // Render the game
        game.render(&mut ball, &paddle1, &paddle2, window_size);
        let loop_duration: u128 = SystemTime::now()
            .duration_since(loop_start_time)
            .expect("Couldn't get time difference.")
            .as_micros();
        std::thread::sleep(std::time::Duration::from_micros(
            ((1000000 / 60) as u64).saturating_sub(loop_duration as u64),
        ));
    }
}
