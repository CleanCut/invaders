use crate::frame::{Drawable, Frame};

pub struct Lives {
    lives: usize,
}

impl Lives {
    pub fn new() -> Self {
        Self { lives: 3 }
    }

    pub fn lose_life(&mut self) -> bool {
        if self.lives > 0 {
            self.lives -= 1;
        }
        self.lives > 0
    }
}

impl Drawable for Lives {
    fn draw(&self, frame: &mut Frame) {
        // format our lives string
        let formatted = format!("LIVES: {:0>2}", self.lives);

        // iterate over all characters
        for (i, c) in formatted.chars().enumerate() {
            // put them in the first row
            frame[i + 12][0] = c;
        }
    }
}
