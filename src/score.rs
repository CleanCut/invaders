use crate::frame::{Drawable, Frame};

pub struct Score {
    count: u16,
}

impl Score {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn add_points(&mut self, amount: u16) {
        self.count += amount;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        // format our score string
        let formated = format!("SCORE: {:0>4}", self.count);

        // create a vector of chars to write each score char into
        let mut chars = Vec::<char>::new();

        // copy chars from formated string into the char vector
        for c in formated.chars() {
            chars.push(c);
        }

        // iterate over all characters
        for (i, c) in chars.iter().enumerate() {
            // put them in the first row
            frame[i][0] = *c;
        }
    }
}
