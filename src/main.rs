use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable, Frame};
use invaders::invaders::Invaders;
use invaders::menu::Menu;
use invaders::player::Player;
use invaders::score::Score;
use invaders::Scenes;
use invaders::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use std::{io, thread};

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    loop {
        let curr_frame = match render_rx.recv() {
            Ok(x) => x,
            Err(_) => break,
        };
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    let audios = ["explode", "lose", "move", "pew", "startup", "win"];
    for item in audios.iter() {
        audio.add(item, &format!("{}.wav", item));
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
    let mut scene = Scenes::Menu;

    fn reset_game(scene: &mut Scenes, player: &mut Player, invaders: &mut Invaders) {
        *scene = Scenes::Menu;
        *player = Player::new();
        *invaders = Invaders::new();
    }

    fn draw(drawables: Vec<&dyn Drawable>, frame: &mut Frame) {
        for drawable in drawables {
            drawable.draw(frame);
        }
    }

    while scene != Scenes::End {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if scene == Scenes::Menu {
            // How can I improve this? (used to avoid problems with references)
            if menu.selected == Scenes::Game {
                scene = Scenes::Game;
                menu.reset_selected();
            } else if menu.selected == Scenes::End {
                scene = Scenes::End;
                menu.reset_selected();
            }
            // Input
            menu.set_handlers()?;
            // Draw & render
            draw(vec![&menu], &mut curr_frame);
        } else if scene == Scenes::Game {
            // How can I improve this? (used to avoid problems with references)
            if player.scene == Scenes::Menu {
                reset_game(&mut scene, &mut player, &mut invaders);
            }
            // Input
            player.set_handlers(&mut audio)?;

            // Updates
            player.update(delta);
            if invaders.update(delta) {
                audio.play("move");
            }
            let hits: u16 = player.detect_hits(&mut invaders);
            if hits > 0 {
                audio.play("explode");
                score.add_points(hits);
            }
            // Draw & render
            draw(vec![&player, &invaders, &score], &mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Final checks if we are in game
        if scene == Scenes::Game {
            // Win or lose?
            if invaders.all_killed() {
                audio.play("win");
                reset_game(&mut scene, &mut player, &mut invaders);
            }
            if invaders.reached_bottom() {
                audio.play("lose");
                reset_game(&mut scene, &mut player, &mut invaders);
            }
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
