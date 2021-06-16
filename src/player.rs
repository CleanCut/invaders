use crate::Scenes;
use crate::frame::{Drawable, Frame};
use crate::invaders::Invaders;
use crate::shot::Shot;
use crate::{NUM_COLS, NUM_ROWS};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crossterm::{event};
use std::error::Error;
use rusty_audio::Audio;


pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    pub scene: Scenes,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
            scene: Scenes::Game,
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
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }
    pub fn set_handlers(&mut self,  audio: &mut Audio) -> Result<(), Box<dyn Error>> {
        // Inputs
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => self.move_left(),
                    KeyCode::Right => self.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if self.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        self.scene = Scenes::Menu;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = String::from("A");
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
