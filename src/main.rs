use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

        // Main game code
        circle.check_colliding(&window);
        circle.update();
        circle.draw(&mut canvas);

        canvas.present();
    }
}

struct Circle {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
    r: u32,
    color: Color,
}

impl Circle {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);

        let mut x = 0;
        let mut y = self.r as i32;
        let mut d = 1 - self.r as i32;

        while x <= y {
            // Draw 8 octants of the circle for symmetry
            for &point in &[
                (self.x + x, self.y + y),
                (self.x - x, self.y + y),
                (self.x + x, self.y - y),
                (self.x - x, self.y - y),
                (self.x + y, self.y + x),
                (self.x - y, self.y + x),
                (self.x + y, self.y - x),
                (self.x - y, self.y - x),
            ] {
                canvas.fill_rect(Rect::new(point.0, point.1, 1, 1)).unwrap();
            }

            // Update Bresenham algorithm values
            x += 1;
            if d < 0 {
                d += 2 * x + 1;
            } else {
                y -= 1;
                d += 2 * (x - y) + 1;
            }
        }
    }

    fn check_colliding(&mut self, window: &Window) {
        let (width, height) = window.size();
        if (self.x < 0) || (self.x + self.r as i32 > width as i32) {
            self.x_vel *= -1;
            self.x = self.x.clamp(self.r as i32, width as i32 - self.r as i32);
        }
        if (self.y < 0) || (self.y + self.r as i32 > height as i32) {
            self.y_vel *= -1;
            self.y = self.y.clamp(self.r as i32, width as i32 - self.r as i32);
        }
    }

    fn update(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}
