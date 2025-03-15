use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Circle {
   pub x: i32,
   pub y: i32,
   pub x_vel: i32,
   pub y_vel: i32,
   pub r: u32,
   pub color: Color,
}

impl Circle {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
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

    pub fn check_colliding(&mut self, window: &Window) {
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

    pub fn update(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}
