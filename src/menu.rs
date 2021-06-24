use crate::frame::{Drawable, Frame};
use crate::Scenes;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::error::Error;
use std::time::Duration;

pub struct Menu {
    options: Vec<String>,
    pub selection: usize,
    pub selected: Scenes,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![String::from("New game"), String::from("Exit")],
            selection: 0,
            selected: Scenes::Menu,
        }
    }

    pub fn reset_selected(&mut self) {
        self.selected = Scenes::Menu;
    }

    pub fn change_option(&mut self, upwards: bool) {
        if upwards && self.selection > 0 {
            self.selection -= 1;
        } else if !upwards && self.selection <= self.options.len() - 1 {
            self.selection += 1;
        }
    }

    pub fn select_option(&mut self) {
        if self.selection == 0 {
            self.selected = Scenes::Game;
        } else {
            self.selected = Scenes::End;
        }
    }

    pub fn set_handlers(&mut self) -> Result<(), Box<dyn Error>> {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => self.change_option(true),
                    KeyCode::Down => self.change_option(false),
                    KeyCode::Char(' ') | KeyCode::Enter => self.select_option(),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

// Reuse Frame grid to print the menu options
impl Drawable for Menu {
    fn draw(&self, frame: &mut Frame) {
        frame[0][self.selection] = '>';
        for (index, option) in self.options.iter().enumerate() {
            for i in 0..option.len() {
                frame[i + 1][index] = self.options[index].chars().nth(i).unwrap();
            }
        }
    }
}
