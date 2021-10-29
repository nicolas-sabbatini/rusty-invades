use crate::bullet::{Bullet, Owner};
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rand::rngs::ThreadRng;
use rand::Rng;
use rusty_audio::Audio;
use rusty_time::timer::Timer;
use std::cmp::max;
use std::time::Duration;

// Individual aliens
pub struct Alien {
  x: usize,
  y: usize,
}

impl Alien {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }

  pub fn fire(&self) -> Bullet {
    Bullet::new(self.x, self.y + 1, Owner::Aliens)
  }
}

// Group of aliens
pub struct Army {
  pub aliens: Vec<Alien>,
  bullets: Vec<Bullet>,
  move_timer: Timer,
  move_rate: u64,
  fire_timer: Timer,
  fire_rate: u64,
  direction: Direction,
}

pub enum ArmyDensity {
  All,
  Odd,
  Even,
}

enum Direction {
  Left,
  Right,
}

impl Army {
  pub fn new(density: ArmyDensity, rows: usize) -> Self {
    // Create a empty vec of aliens
    let mut aliens = Vec::new();
    // We don´t want the aliens touching the borders
    for x in 2..NUM_COLS - 2 {
      let mut rows = rows;
      // Left space for UI
      for y in 1..NUM_ROWS {
        // If all the aliens of the row are already in place go to nex row
        if rows <= 0 {
          break;
        }
        // Else create a new alien
        match density {
          ArmyDensity::All => {
            if rows > 0 {
              aliens.push(Alien::new(x, y));
              rows -= 1;
            }
          }
          ArmyDensity::Odd => {
            if (rows > 0) && (y % 2 == 1) && (x % 2 == 1) {
              aliens.push(Alien::new(x, y));
              rows -= 1;
            }
          }
          ArmyDensity::Even => {
            if (rows > 0) && (y % 2 == 0) && (x % 2 == 0) {
              aliens.push(Alien::new(x, y));
              rows -= 1;
            }
          }
        }
      }
    }

    Self {
      aliens,
      bullets: Vec::new(),
      move_timer: Timer::from_millis(2000),
      move_rate: 2000,
      fire_timer: Timer::from_millis(2000),
      fire_rate: 2000,
      direction: Direction::Right,
    }
  }

  pub fn update(&mut self, delta: Duration, audio: &mut Audio, rng: &mut ThreadRng) {
    // Move aliens
    self.move_timer.update(delta);
    if self.move_timer.ready {
      // Reset Timer
      self.move_timer.reset();
      // Play audio
      audio.play("move");
      // Check if we need to go down
      let mut down = false;
      // Change direction
      match self.direction {
        Direction::Left => {
          let min_x = self.aliens.iter().map(|alien| alien.x).min().unwrap_or(0);
          if min_x <= 0 {
            self.direction = Direction::Right;
            down = true;
          }
        }
        Direction::Right => {
          let max_x = self
            .aliens
            .iter()
            .map(|alien| alien.x)
            .max()
            .unwrap_or(NUM_COLS - 1);
          if max_x >= NUM_COLS - 1 {
            self.direction = Direction::Left;
            down = true;
          }
        }
      }
      // Go down or side to side
      if down {
        self.move_rate = max(250, self.move_rate - 250);
        self.move_timer = Timer::from_millis(self.move_rate);
        for alien in self.aliens.iter_mut() {
          alien.y += 1;
        }
      } else {
        for alien in self.aliens.iter_mut() {
          match self.direction {
            Direction::Left => alien.x -= 1,
            Direction::Right => alien.x += 1,
          }
        }
      }
    }

    // Fire guns
    self.fire_timer.update(delta);
    if self.fire_timer.ready && !self.aliens.is_empty() {
      // Update Fire rate
      self.fire_rate = max(400, self.fire_rate - 50);
      self.fire_timer = Timer::from_millis(self.fire_rate);
      // A random alien fire
      let new_bullet = self.aliens[rng.gen_range(0..self.aliens.len())].fire();
      self.bullets.push(new_bullet);
      // Play fire sound
      audio.play("pew_1");
    }

    // Update bullet position
    for bullet in self.bullets.iter_mut() {
      bullet.update(delta);
    }
    self.bullets.retain(|bullet| !bullet.ready_to_clear())
  }

  pub fn all_dead(&self) -> bool {
    self.aliens.is_empty()
  }

  pub fn reach_bottom(&self) -> bool {
    self.aliens.iter().map(|alien| alien.y).max().unwrap_or(0) >= NUM_ROWS - 2
  }

  pub fn can_kill_alien(&mut self, x: usize, y: usize) -> bool {
    if let Some(alien_pos) = self
      .aliens
      .iter()
      .position(|alien| alien.x == x && alien.y == y)
    {
      self.aliens.remove(alien_pos);
      return true;
    };
    return false;
  }

  pub fn is_colliding_with_bullet(&self, x: usize, y: usize) -> bool {
    if let Some(_bullet_pos) = self
      .bullets
      .iter()
      .position(|bullet| bullet.x == x && bullet.y == y)
    {
      return true;
    };
    return false;
  }
}

impl Drawable for Army {
  fn draw(&self, frame: &mut Frame) {
    // Draw aliens
    for alien in self.aliens.iter() {
      frame[alien.x][alien.y] =
        if (self.move_timer.time_left.as_millis() as f64 / self.move_rate as f64) >= 0.5 {
          "x"
        } else {
          "+"
        }
    }

    // Draw bullets
    for bullet in self.bullets.iter() {
      bullet.draw(frame);
    }
  }
}
