use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::game::Game;
use super::paddle::Paddle;

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub r: u32,
    pub color: Color,
}

impl Ball {
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

    pub fn check_colliding(&mut self, paddles: [&Paddle; 2], game: &mut Game, window: &Window) {
        let (width, height) = window.size();

        // Left and right wall collisions
        if (self.x + self.r as i32) <= 0 {
            self.x_vel *= -1;
            self.y_vel = 5;
            self.x = width as i32 / 2;
            self.y = height as i32 / 2;
            game.increase_score(2);
        } else if (self.x + self.r as i32) > width as i32 {
            self.x_vel *= -1;
            self.y_vel = 3;
            self.x = width as i32 / 2;
            self.y = height as i32 / 2;
            game.increase_score(1);
        }
        // Top and bottom wall collisions
        if (self.y < 0) || (self.y + self.r as i32 > height as i32) {
            self.y_vel *= -1;
            self.y = self.y.clamp(self.r as i32, width as i32 - self.r as i32);
        }

        for paddle in paddles.iter() {
            // Get paddle boundaries
            let left_side = paddle.x;
            let right_side = paddle.x + paddle.width as i32;
            let top_side = paddle.y;
            let bottom_side = paddle.y + paddle.height as i32;

            // Get approaching side
            let closest_x = self.x.clamp(left_side, right_side);
            let closest_y = self.y.clamp(top_side, bottom_side);

            // Get distance from circle centre to closest paddle side
            let distance = ((closest_x - self.x).pow(2) + (closest_y - self.y).pow(2)).isqrt();
            // True if colliding
            if distance <= self.r as i32 {
                self.x_vel *= -1;
                self.y_vel += (self.y - (paddle.y + (paddle.height as i32 / 2))) / 10;
            }
        }
    }

    pub fn update(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}
