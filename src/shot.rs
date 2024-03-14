use crate::{
    frame::{Drawable, Frame},
    NUM_ROWS, PLAYER_CHAR_HEIGHT,
};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        let bottom_bound_y = NUM_ROWS - PLAYER_CHAR_HEIGHT;
        let pos_y = if self.y >= bottom_bound_y {
            bottom_bound_y
        } else {
            self.y
        };

        frame[self.x][pos_y] = if self.exploding { '*' } else { '|' };
    }
}
