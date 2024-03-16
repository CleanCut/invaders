use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error, io, sync::mpsc::{self, Receiver}, thread::{self}, time::{Duration, Instant}
};

use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::Player,
    render,
    score::Score,
    rusty_bot::Agent,
};

const RUSTY_BOT_MAX_GAMES_TO_LEARN: i32 = 50;

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn reset_game(in_menu: &mut bool, player: &mut Player, invaders: &mut Invaders, rusty_bot: bool, game_number: i32) {
    if rusty_bot && game_number < RUSTY_BOT_MAX_GAMES_TO_LEARN {
        *in_menu = false;
    } 
    else {
        *in_menu = true;
    }
    *player = Player::new();
    *invaders = Invaders::new();
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
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    let mut score = Score::new();
    let mut menu = Menu::new();
    let mut in_menu = true;
    let mut level = Level::new();
    let mut rusty_bot: bool = false;
    let mut agent: Agent = Agent::new(0.05, 0.9);
    let mut game_number = 1;
    let mut current_state;
    let mut action;
    let mut reward = 0.0;

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
                            } else if menu.selection == 1 {
                                in_menu = false;
                                rusty_bot = true;
                            }
                            else {
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
        current_state = agent.get_state(&mut invaders, &mut player);
        action = agent.act(current_state, game_number);
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        reset_game(&mut in_menu, &mut player, &mut invaders, false, 0);
                    }
                    _ => {}
                }
            }
        }

        if rusty_bot {
            // If allowed bot to play let bot decide what to play.
            match action {
                0 => player.move_left(),
                1 => player.move_right(),
                2 => {player.shoot();},
                _ => {}
            }
        }
        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        let hits: u16 = player.detect_hits(&mut invaders);
        if hits > 0 {
            audio.play("explode");
            score.add_points(hits);
            // % of hits of remaining army
            // reward += (hits as f32) / ((1.0 + invaders.army.len() as f32) - hits as f32);
            reward += 0.05;
        }
        else {
            reward -= 0.01;
        }
        // Draw & render

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            if level.increment_level() {
                audio.play("win");
                // reward += 1.0;
                let new_state = agent.get_state(&mut invaders, &mut player);
                agent.learn(current_state, action, reward, new_state);
                break 'gameloop;
            }
            invaders = Invaders::new();
        } else if invaders.reached_bottom() {
            // reward -= 1.0;
            audio.play("lose");
            reset_game(&mut in_menu, &mut player, &mut invaders, rusty_bot, game_number);
            game_number += 1;
        }
        let new_state = agent.get_state(&mut invaders, &mut player);
        agent.learn(current_state, action, reward, new_state);
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
