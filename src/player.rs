use crate::alien::Army;
use crate::bullet::{Bullet, Owner};
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rand::rngs::ThreadRng;
use rand::Rng;
use rusty_audio::Audio;
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Player {
  pub x: usize,
  pub y: usize,
  bullets: Vec<Bullet>,
  shot_timer: Timer,
  pub alive: bool,
  score: u32,
  lives: u8,
}

impl Player {
  pub fn new() -> Self {
    Self {
      x: NUM_COLS / 2,
      y: NUM_ROWS - 2,
      bullets: Vec::new(),
      shot_timer: Timer::from_millis(200),
      alive: true,
      score: 0,
      lives: 3,
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

  pub fn shot(&mut self, audio: &mut Audio, rng: &mut ThreadRng) {
    if self.shot_timer.ready {
      self
        .bullets
        .push(Bullet::new(self.x, self.y - 1, Owner::Player));
      let random_pew = "pew_".to_owned() + &rng.gen_range(2..=3).to_string();
      audio.play(&random_pew);
      self.shot_timer.reset();
    }
  }

  pub fn update(
    &mut self,
    delta: Duration,
    mut army: &mut Army,
    mut audio: &mut Audio,
    mut rng: &mut ThreadRng,
  ) -> u32 {
    self.shot_timer.update(delta);
    for bullet in self.bullets.iter_mut() {
      bullet.update(delta);
    }
    self.bullets.retain(|bullet| !bullet.ready_to_clear());

    self.detect_kills(&mut army, &mut audio, &mut rng);
    self.score
  }

  pub fn can_kill(&self, x: usize, y: usize) -> bool {
    self.x == x && self.y == y
  }

  pub fn detect_kills(&mut self, army: &mut Army, audio: &mut Audio, rng: &mut ThreadRng) {
    for bullet in self.bullets.iter_mut() {
      if army.can_kill_alien(bullet.x, bullet.y) {
        bullet.explode();
        let random_boom = "boom_".to_owned() + &rng.gen_range(2..=3).to_string();
        audio.play(&random_boom);
        self.score += 100
      }
    }
  }

  pub fn kill(&mut self) {
    self.alive = false;
  }
}

impl Drawable for Player {
  fn draw(&self, frame: &mut Frame) {
    frame[self.x][self.y] = "A";
    for bullet in self.bullets.iter() {
      bullet.draw(frame);
    }
  }
}
