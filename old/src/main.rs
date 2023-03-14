use crossbeam::channel::unbounded;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use rand::thread_rng;
use rusty_audio::Audio;
use rusty_invades::alien::{Army, ArmyDensity};
use rusty_invades::frame::{new_frame, Drawable};
use rusty_invades::player::Player;
use rusty_invades::render::render;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    // Create random generator
    let mut rng = thread_rng();

    // Create audio
    let mut audio = Audio::new();
    audio.add("start", "assets/start.wav");
    audio.add("dead", "assets/dead.wav");
    audio.add("move", "assets/move.wav");
    audio.add("win", "assets/win.wav");
    audio.add("pew_1", "assets/pew_1.wav");
    audio.add("pew_2", "assets/pew_2.wav");
    audio.add("pew_3", "assets/pew_3.wav");
    audio.add("boom_1", "assets/boom_1.wav");
    audio.add("boom_2", "assets/boom_2.wav");
    audio.add("boom_3", "assets/boom_3.wav");

    // Create terminal
    let mut std_out = io::stdout();
    terminal::enable_raw_mode()?;
    std_out.execute(EnterAlternateScreen)?;
    std_out.execute(Hide)?;

    // Play start sound
    audio.play("start");

    // Create render channel
    let (render_sender, render_receiver) = unbounded();
    // Create render thread
    let render_thread = thread::spawn(move || {
        let mut previous_frame = new_frame();
        let mut std_out = io::stdout();
        render(&mut std_out, &previous_frame, &previous_frame, 0, true);
        loop {
            let (new_frame, score) = match render_receiver.recv() {
                Ok(frame_data) => frame_data,
                Err(_) => break,
            };
            render(&mut std_out, &previous_frame, &new_frame, score, false);
            previous_frame = new_frame;
        }
    });

    // Create player
    let mut player = Player::new();

    // Create evil aliens
    let mut evil_aliens = Army::new(ArmyDensity::Even, 3);

    // Create delta time
    let mut now = Instant::now();

    // Game Loop
    'game_loop: loop {
        // Calculate delta time
        let delta = now.elapsed();
        now = Instant::now();

        // Event handle
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'game_loop,
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => player.shot(&mut audio, &mut rng),
                    _ => (),
                }
            }
        }

        // Update
        let current_score = player.update(delta, &mut evil_aliens, &mut audio, &mut rng);
        evil_aliens.update(delta, &mut audio, &mut rng);

        if evil_aliens.is_colliding_with_bullet(player.x, player.y) {
            player.kill();
            audio.play("boom_3");
        }
        // Draw
        // Create new frame
        let mut new_frame = new_frame();
        // Draw player and stuff
        let drawables: Vec<&dyn Drawable> = vec![&player, &evil_aliens];
        for drawable in drawables.iter() {
            drawable.draw(&mut new_frame);
        }

        // Send frame to thread
        let _ = render_sender.send((new_frame, current_score));
        sleep(Duration::from_millis(1));

        if evil_aliens.all_dead() {
            audio.play("win");
            break 'game_loop;
        }

        if evil_aliens.reach_bottom() || !player.alive {
            audio.wait();
            audio.play("dead");
            break 'game_loop;
        }
    }

    // Await for audio tread
    audio.wait();

    // Clean up
    // Drop channel
    drop(render_sender);
    // Drop thread
    render_thread.join().unwrap();
    // Show cursor
    std_out.execute(Show)?;
    // Enter normal screen
    std_out.execute(LeaveAlternateScreen)?;
    // Enter normal mode
    terminal::disable_raw_mode()?;
    Ok(())
}
