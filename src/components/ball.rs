use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::game::Game;
use super::paddle::{Paddle, Paddles};

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub r: u32,
    pub color: Color,
    explode_count: i8,
}

impl Ball {
    pub fn new(x: i32, y: i32, r: u32) -> Self {
        Ball {
            x,
            y,
            r,
            x_vel: 8,
            y_vel: 6,
            color: Color::WHITE,
            explode_count: -1,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, window_size: (i32, i32)) {
        self.explode_count = self.explode_count.saturating_sub(1);
        canvas.set_draw_color(self.color);

        match self.explode_count {
            0 => {
                self.x = window_size.0 / 2;
                self.y = window_size.1 / 2;
            }
            n if n < 0 => {
                let mut x: i32 = 0;
                let mut y: i32 = self.r as i32;
                let mut d: i32 = 1 - self.r as i32;

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
                        canvas
                            .fill_rect(Rect::new(point.0, point.1, 1, 1))
                            .expect("Couldn't draw the ball.");
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
            _ => {
                let r: i32 = self.r as i32 + (60 / (self.explode_count as i32));
                let y: i32 = self.y;
                let x: i32 = self.x;
                for point in [
                    (x, y - r),
                    (
                        x + ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                        y - ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                    ),
                    (x + r, y),
                    (
                        x + ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                        y + ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                    ),
                    (x, y + r),
                    (
                        x - ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                        y - ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                    ),
                    (x - r, y),
                    (
                        x - ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                        y + ((2.0_f32).sqrt() * r as f32 / 2.0) as i32,
                    ),
                ] {
                    canvas
                        .fill_rect(Rect::new(point.0, point.1, 3, 3))
                        .expect("Couldn't draw exploding part.");
                }
            }
        }
    }

    pub fn check_colliding(&mut self, paddles: [&Paddle; 2], game: &mut Game, window: &Window) {
        if self.explode_count > 0 {
            return;
        }

        let (width, height) = window.size();

        // Left and right wall collisions
        if (self.x - self.r as i32) <= 0 {
            self.x_vel *= -1;
            self.y_vel = 5;
            game.increase_score(Paddles::Paddle2, self);
        } else if (self.x + self.r as i32) >= width as i32 {
            self.x_vel *= -1;
            self.y_vel = 3;
            game.increase_score(Paddles::Paddle1, self);
        }
        // Top and bottom wall collisions
        if (self.y < 0) || (self.y + self.r as i32 > height as i32) {
            self.y_vel *= -1;
            self.y = self.y.clamp(self.r as i32, width as i32 - self.r as i32);
        }

        for paddle in paddles.iter() {
            // Get paddle boundaries
            let left_side: i32 = paddle.x;
            let right_side: i32 = paddle.x + paddle.width as i32;
            let top_side: i32 = paddle.y;
            let bottom_side: i32 = paddle.y + paddle.height as i32;

            // Get approaching side
            let closest_x: i32 = self.x.clamp(left_side, right_side);
            let closest_y: i32 = self.y.clamp(top_side, bottom_side);

            // Get distance from circle centre to closest paddle side
            let distance: i32 = ((closest_x - self.x).pow(2) + (closest_y - self.y).pow(2)).isqrt();
            // True if colliding
            if distance <= self.r as i32 {
                self.x_vel *= -1;
                self.y_vel += (self.y - (paddle.y + (paddle.height as i32 / 2))) / 10;
            }
        }
    }

    pub fn update_movement(&mut self) {
        if self.explode_count <= 0 {
            self.x += self.x_vel;
            self.y += self.y_vel;
        }
    }

    pub fn explode(&mut self) {
        self.explode_count = 60;
    }
}
