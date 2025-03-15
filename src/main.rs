use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    let mut ball = Ball{x: 100.0, y: 100.0, r: 10.0, x_vel: 5.0, y_vel: 5.0, color: WHITE};
    loop {
        clear_background(BLACK);

        draw_circle(ball.x, ball.y, ball.r, ball.color);
        ball.x += ball.x_vel;
        ball.y += ball.y_vel;
        check_wall_collisions(&mut ball);

        next_frame().await
    }
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    x_vel: f32,
    y_vel: f32,
    color: Color
}

fn check_wall_collisions(ball: &mut Ball) {
    let x = ball.x;
    let y = ball.y;
    let r = ball.r;

    if x < (0.0 + r) || x > (screen_width() - r) {
        ball.x_vel *= -1.0;
    }
    if y < (0.0 + r) || y > (screen_height() - r) {
        ball.y_vel *= -1.0;
    }
}