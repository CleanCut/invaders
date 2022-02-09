pub const SOUND_STARTUP: &str = "startup";
pub const SOUND_SHOT: &str = "pew";
pub const SOUND_LOSE: &str = "lose";
pub const SOUND_EXPLOSION: &str = "explode";
pub const SOUND_MOVE: &str = "move";
pub const SOUND_WIN: &str = "win";

pub const SOUND_FILE_NAMES: [&str; 6] = [
    SOUND_STARTUP,
    SOUND_SHOT,
    SOUND_LOSE,
    SOUND_EXPLOSION,
    SOUND_MOVE,
    SOUND_WIN,
];

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

pub const MAX_LEVEL: u8 = 3;

pub const PLAYER_MAX_SIMULTANEOUS_SHOTS: usize = 2;

pub const INVADERS_DIRECTION_LEFT: i32 = -1;
pub const INVADERS_DIRECTION_RIGHT: i32 = 1;
pub const INVADERS_FIRST_ROW: usize = 2;
pub const INVADERS_ROW_COUNT: usize = 4;
pub const INVADERS_ROW_INTERVAL: usize = 2;
pub const INVADERS_LAST_ROW: usize =
    INVADERS_FIRST_ROW + (INVADERS_ROW_COUNT - 1) * INVADERS_ROW_INTERVAL;
pub const INVADERS_FIRST_COL: usize = 1; // Start on odd column, so player cannot shoot immediately
pub const INVADERS_COL_INTERVAL: usize = INVADERS_ROW_INTERVAL;
pub const INVADERS_LAST_COL: usize = NUM_COLS - 3;
pub const INVADER_FACES: [[char; 2]; INVADERS_ROW_COUNT] =
    [['x', '+'], [')', '('], ['O', '0'], ['_', '-']];

// Timings
pub const KEYBOARD_POLLING_TIMEOUT_MS: u64 = 2;
pub const RENDER_INTERVAL_MS: u64 = 5;
pub const INVADERS_INITAL_MOVE_INTERVAL_MS: u64 = 2_000;
pub const INVADERS_MOVE_INTERVAL_DECREMENT_MS: u64 = 250;
pub const INVADERS_MOVE_INTERVAL_MINIMAL_MS: u64 = 250;
pub const SHOT_MOVE_INTERVAL_MS: u64 = 45;
pub const SHOT_EXPLOSION_DURATION_MS: u64 = 260;
    