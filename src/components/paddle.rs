use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub vel: i32,
    height: u32,
    width: u32,
    color: Color,
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Self {
        Paddle {
            x,
            y,
            vel: 8,
            height: 100,
            width: 20,
            color: Color::WHITE,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas
            .draw_rect(Rect::new(self.x, self.y, self.width, self.height))
            .unwrap();
    }

    pub fn move_paddle(&mut self, amount: i32) {
        self.y += amount;
    }
}
