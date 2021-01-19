use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
};

pub struct Score {
    chars: Vec<char>,
}

impl Score {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }

    pub fn update(&mut self, invaders: &Invaders) {
        // format our score string
        let formated = format!("SCORE: {:0>3}", invaders.total_count - invaders.army.len());

        // clear the old score vector
        self.chars.clear();

        // copy chars from formated string into the char vector
        for c in formated.chars() {
            self.chars.push(c);
        }
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        // iterate over all characters
        for (i, c) in self.chars.iter().enumerate() {
            // put them in the first row
            frame[i][0] = *c;
        }
    }
}
