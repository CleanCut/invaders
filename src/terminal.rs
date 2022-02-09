use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, Stdout};

pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn start() -> io::Result<Terminal> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;

        Ok(Terminal { stdout })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.stdout.execute(Show).unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap_or(());
    }
}
