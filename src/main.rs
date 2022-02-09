use crossterm::event::{self, Event, KeyCode};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use invaders::{
    constants::{
        KEYBOARD_POLLING_TIMEOUT_MS, RENDER_INTERVAL_MS, SOUND_EXPLOSION, SOUND_FILE_NAMES,
        SOUND_LOSE, SOUND_MOVE, SOUND_SHOT, SOUND_STARTUP, SOUND_WIN,
    },
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::Player,
    render,
    score::Score,
    terminal::Terminal,
};

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn reset_game(in_menu: &mut bool, player: &mut Player, invaders: &mut Invaders) {
    *in_menu = true;
    *player = Player::new();
    *invaders = Invaders::new();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    for item in &SOUND_FILE_NAMES {
        audio.add(item, &format!("{}.wav", item));
    }
    audio.play(SOUND_STARTUP);

    // Terminal
    let _terminal = Terminal::start()?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        render_screen(render_rx);
    });

    // Game loop
    let mut player: Player = Default::default();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    let mut score = Score::new();
    let mut menu = Menu::new();
    let mut in_menu = true;
    let mut level = Level::new();

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if in_menu {
            // Input handlers for the menu
            while event::poll(Duration::from_millis(KEYBOARD_POLLING_TIMEOUT_MS))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Up => menu.change_option(true),
                        KeyCode::Down => menu.change_option(false),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if menu.selection == 0 {
                                in_menu = false;
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
            thread::sleep(Duration::from_millis(RENDER_INTERVAL_MS));
            continue;
        }

        // Input handlers for the game
        while event::poll(Duration::from_millis(KEYBOARD_POLLING_TIMEOUT_MS))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play(SOUND_SHOT);
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play(SOUND_LOSE);
                        reset_game(&mut in_menu, &mut player, &mut invaders);
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play(SOUND_MOVE);
        }
        let hits: u16 = player.detect_hits(&mut invaders);
        if hits > 0 {
            audio.play(SOUND_EXPLOSION);
            score.add_points(hits);
        }
        // Draw & render

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(RENDER_INTERVAL_MS));

        // Win or lose?
        if invaders.all_killed() {
            if level.increment_level() {
                audio.play(SOUND_WIN);
                break 'gameloop;
            }
            invaders = Invaders::new();
        } else if invaders.reached_bottom() {
            audio.play(SOUND_LOSE);
            reset_game(&mut in_menu, &mut player, &mut invaders);
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    Ok(())
}
