use crate::{
    frame::{Drawable, Reset, Frame},
    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    view: char,
}

impl Player {
    pub fn new(view: char) -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
            view: view
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new('A')
    }
}

impl Reset for Player {
    fn reset(&mut self) {
        *self = Player::new(self.view);
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = self.view;
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
#[derive(PartialEq, Debug)]
pub enum Player2Mode {
    Enabled(bool),
}

impl Drawable for Player2Mode {
    fn draw(&self, frame: &mut Frame) {
        // format our player2 string
        if *self == Player2Mode::Enabled(false) {
            let formatted = format!("P2: PRESS E");

            // iterate over all characters
            for (i, c) in formatted.chars().enumerate() {
                // put them in the first row
                frame[i + 23][0] = c;
            }
        }
    }
}
