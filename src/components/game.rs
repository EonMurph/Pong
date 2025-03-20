use super::paddle::Paddles;

pub struct Game {
    p1_score: u8,
    p2_score: u8,
    winning_score: u8,
    pub freeze: bool, // temp variable just for testing game over
}

impl Game {
    pub fn new(winning_score: u8) -> Self {
        Game {
            p1_score: 0,
            p2_score: 0,
            winning_score,
            freeze: false,
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
        self.freeze = true;
    }
}
