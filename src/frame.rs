use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = [[char; NUM_ROWS]; NUM_COLS];

pub fn new_frame() -> Frame {
    [[' '; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
