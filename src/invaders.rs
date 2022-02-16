use crate::{
    constants::*,
    frame::{Drawable, Frame},
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};

struct Invader {
    pub x: usize,
    pub y: usize,
    points: u16,
    face1: char,
    face2: char,
}

pub struct Invaders {
    army: Vec<Invader>,
    pub total_count: usize,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for (y, faces) in (INVADERS_FIRST_ROW..=INVADERS_LAST_ROW)
            .step_by(INVADERS_ROW_INTERVAL)
            .zip(INVADER_FACES.iter())
        {
            // unwrap is safe as the INVADER_FACES is guarded by INVADERS_ROW_COUNT
            for x in (INVADERS_FIRST_COL..INVADERS_LAST_COL).step_by(INVADERS_COL_INTERVAL) {
                army.push(Invader {
                    x,
                    y,
                    points: 1,
                    face1: faces[0],
                    face2: faces[1],
                });
            }
        }
        let total_count = army.len();
        Self {
            army,
            total_count,
            move_timer: Timer::from_millis(INVADERS_INITAL_MOVE_INTERVAL_MS),
            direction: INVADERS_DIRECTION_RIGHT,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == INVADERS_DIRECTION_LEFT {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = INVADERS_DIRECTION_RIGHT;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = INVADERS_DIRECTION_LEFT;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(
                    self.move_timer.duration.as_millis() as u64
                        - INVADERS_MOVE_INTERVAL_DECREMENT_MS,
                    INVADERS_MOVE_INTERVAL_MINIMAL_MS,
                );
                self.move_timer = Timer::from_millis(new_duration);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> u16 {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            let points = self.army[idx].points;
            self.army.remove(idx);
            points
        } else {
            0
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                invader.face1
            } else {
                invader.face2
            }
        }
    }
}
