pub mod frame;
pub mod invaders;
pub mod menu;
pub mod player;
pub mod render;
pub mod score;
pub mod shot;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

#[derive(PartialEq, Copy, Clone)]
pub enum Scenes {
    Menu,
    Game,
    End,
}
