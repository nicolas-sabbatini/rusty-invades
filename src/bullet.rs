use crate::frame::{Drawable, Frame};
use crate::NUM_ROWS;
use rusty_time::timer::Timer;
use std::time::Duration;

pub enum Owner {
  Player,
  Aliens,
}

pub struct Bullet {
  pub x: usize,
  pub y: usize,
  alive: bool,
  timer: Timer,
  owner: Owner,
}

impl Bullet {
  pub fn new(x: usize, y: usize, owner: Owner) -> Self {
    Self {
      x,
      y,
      alive: true,
      timer: Timer::from_millis(50),
      owner,
    }
  }

  pub fn update(&mut self, delta: Duration) {
    self.timer.update(delta);
    if self.timer.ready && self.alive {
      match self.owner {
        Owner::Player => self.y -= 1,
        Owner::Aliens => self.y += 1,
      }
      self.timer.reset();
    }
  }

  pub fn explode(&mut self) {
    self.alive = false;
    self.timer = Timer::from_millis(200);
  }

  pub fn ready_to_clear(&self) -> bool {
    match self.owner {
      Owner::Player => (!self.alive && self.timer.ready) || self.y <= 0,
      Owner::Aliens => (!self.alive && self.timer.ready) || self.y >= NUM_ROWS - 1,
    }
  }
}

impl Drawable for Bullet {
  fn draw(&self, frame: &mut Frame) {
    frame[self.x][self.y] = if self.alive { "|" } else { "*" };
  }
}
