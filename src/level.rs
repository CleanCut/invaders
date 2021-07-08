use crate::frame::{Drawable, Frame};

const MAX_LEVEL: u8 = 3;

pub struct Level {
    level: u8,
}

impl Level {
    pub fn new() -> Self {
        Self { level: 1 }
    }

    pub fn increment_level(&mut self) -> bool {
        if self.level <= MAX_LEVEL {
            self.level += 1;
        }
        self.level == MAX_LEVEL
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_increment_level_and_return_false() {
        // given
        let mut level = Level::new();
        // when
        let actual = level.increment_level();
        // then
        assert_eq!(false, actual);
        assert_eq!(2, level.level);
    }

    #[test]
    fn should_increment_level_two_times_and_return_true() {
        // given
        let mut level = Level::new();
        // when
        level.increment_level();
        let actual = level.increment_level();
        // then
        assert_eq!(true, actual);
        assert_eq!(MAX_LEVEL, level.level);
    }
}
