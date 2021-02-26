use std::io::{Stdout, Write};
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;

pub fn render(std_out: &mut Stdout, previous_frame: &Frame, new_frame: &Frame, force: bool) {
  if force {
    std_out.queue(SetBackgroundColor(Color::Blue)).unwrap();
    std_out.queue(Clear(ClearType::All)).unwrap();
    std_out.queue(SetBackgroundColor(Color::Black)).unwrap();
  }
  for (x, col) in new_frame.iter().enumerate() {
    for (y, char) in col.iter().enumerate() {
      if *char != previous_frame[x][y] || force {
        std_out.queue(MoveTo(x as u16, y as u16)).unwrap();
        print!("{}", *char)
      }
    }
  }
  std_out.flush().unwrap();
}