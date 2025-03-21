use super::ball::Ball;
use super::paddle::{Paddle, Paddles};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::{Font, PartialRendering};
use sdl2::video::WindowContext;

pub struct Game {
    pub p1_score: u8,
    pub p2_score: u8,
    winning_score: u8,
    pub over: bool, // temp variable just for testing game over
}

impl Game {
    pub fn new(winning_score: u8) -> Self {
        Game {
            p1_score: 0,
            p2_score: 0,
            winning_score,
            over: false,
        }
    }

    pub fn increase_score(&mut self, paddle: Paddles) {
        match paddle {
            Paddles::Paddle1 => {
                self.p1_score += 1;
                if self.p1_score == self.winning_score {
                    self.game_over();
                }
            }
            Paddles::Paddle2 => {
                self.p2_score += 1;
                if self.p2_score == self.winning_score {
                    self.game_over();
                }
            }
        }
    }

    pub fn game_over(&mut self) {
        self.over = true;
    }

    pub fn render_font(
        &self,
        text: &str,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        canvas: &mut WindowCanvas,
        text_pos: (i32, i32),
        font_size: (u32, u32),
    ) {
        let font_surface: sdl2::surface::Surface = font
            .render(text)
            .solid(Color::WHITE)
            .expect("Couldn't render the font.");
        let texture: sdl2::render::Texture = texture_creator
            .create_texture_from_surface(font_surface)
            .expect("Couldn't create the font texture.");
        let target: Rect = Rect::new(text_pos.0, text_pos.1, font_size.0, font_size.1);
        canvas
            .copy(&texture, None, Some(target))
            .expect("couldn't copy texture to canvas");
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        ball: &Ball,
        paddle1: &Paddle,
        paddle2: &Paddle,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        window_size: (i32, i32),
    ) {
        if self.over {
            let game_over_text: &str = "Game Over";
            let mut game_over_font_size: (u32, u32) = font
                .size_of(game_over_text)
                .expect("Couldn't get font size.");
            game_over_font_size = (game_over_font_size.0 + 10, game_over_font_size.1 + 10);
            self.render_font(
                game_over_text,
                font,
                texture_creator,
                canvas,
                (
                    (window_size.0 / 2) - (game_over_font_size.0 as i32 / 2),
                    (window_size.1 / 2) - (game_over_font_size.1 as i32 / 2),
                ),
                (game_over_font_size.0, game_over_font_size.1),
            );
        } else {
            ball.draw(canvas);
            paddle1.draw(canvas);
            paddle2.draw(canvas);

            let player_score: &str = &format!("{} : {}", self.p1_score, self.p2_score);
            self.render_font(
                player_score,
                font,
                texture_creator,
                canvas,
                (
                    (window_size.0 / 2) - (font.size_of(player_score).unwrap().0 as i32 / 2),
                    20,
                ),
                font.size_of(player_score).expect("Couldn't get font size."),
            );
        }

        canvas.present();
    }
}
