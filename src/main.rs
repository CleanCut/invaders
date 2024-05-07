use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use invaders::{
    frame::{new_frame, Drawable, Reset, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::{Player, Player2Mode},
    render,
    score::Score,
};

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn reset_game(in_menu: &mut bool, player2_mode: &mut Player2Mode,  to_reset: &mut Vec<&mut dyn Reset>) {
    *in_menu = true;
    *player2_mode = Player2Mode::Enabled(false);
    for elem in to_reset {
        elem.reset();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("audio/original/{}.wav", item));
    }
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        render_screen(render_rx);
    });

    // Game loop
    let mut player = Player::new('A');
    let mut player2 = Player::new('Q');
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    let mut score = Score::new();
    let mut menu = Menu::new();
    let mut in_menu = true;
    let mut level = Level::new();
    let mut player2_mode = Player2Mode::Enabled(false);

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if in_menu {
            // Input handlers for the menu
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Up => menu.change_option(true),
                        KeyCode::Down => menu.change_option(false),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if menu.selection == 0 {
                                in_menu = false;
                            }
                            else if menu.selection == 1 {
                                in_menu = false;
                                player2_mode = Player2Mode::Enabled(true);
                            } else {
                                break 'gameloop;
                            }
                        }
                        _ => {}
                    }
                }
            }
            menu.draw(&mut curr_frame);

            let _ = render_tx.send(curr_frame);
            thread::sleep(Duration::from_millis(1));
            continue;
        }

        // Input handlers for the game
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    KeyCode::Char('a') => player2.move_left(),
                    KeyCode::Char('d') => player2.move_right(),
                    KeyCode::Char('w') => {
                        if player2.shoot() {
                            audio.play("pew");
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        let mut to_reset: Vec<&mut dyn Reset> = vec![&mut player, &mut player2, &mut invaders, &mut score, &mut level];
                        reset_game(&mut in_menu, &mut player2_mode, &mut to_reset);
                    },
                    KeyCode::Char('e') => {
                        player2_mode = Player2Mode::Enabled(true);
                    },
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if player2_mode == Player2Mode::Enabled(true) { player2.update(delta); }

        if invaders.update(delta) {
            audio.play("move");
        }
        let mut hits: u16 = player.detect_hits(&mut invaders);
        hits += player2.detect_hits(&mut invaders);
        if hits > 0 {
            audio.play("explode");
            score.add_points(hits);
        }
        // Draw & render

        let mut drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level, &player2_mode];
        if player2_mode == Player2Mode::Enabled(true) {
            drawables.push(&player2);
        }
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            if level.increment_level() {
                audio.play("win");
                break 'gameloop;
            }
            invaders = Invaders::new();
        } else if invaders.reached_bottom() {
            audio.play("lose");
            let mut to_reset: Vec<&mut dyn Reset> = vec![&mut player, &mut player2, &mut invaders, &mut score, &mut level];
            reset_game(&mut in_menu, &mut player2_mode, &mut to_reset);
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
