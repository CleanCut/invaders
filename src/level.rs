use crate::frame::{Drawable, Frame};

pub struct Level {
    level: u8,
}

impl Level {
    pub fn new() -> Self {
        Self { level: 1 }
    }

    pub fn increment_level(&mut self) -> bool {
        if self.level <= 3 {
            self.level += 1;
        }   
        self.level == 3
    }
}

impl Drawable for Level {
    fn draw(&self, frame: &mut Frame) {
        // format our level string
        let formatted = format!("LEVEL: {:0>2}", self.level);

        // iterate over all characters
        for (i, c) in formatted.chars().enumerate() {
            // put them in the first row
            frame[i + 20][0] = c;
        }
    }
}
