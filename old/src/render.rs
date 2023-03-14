use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(
    std_out: &mut Stdout,
    previous_frame: &Frame,
    new_frame: &Frame,
    score: u32,
    force: bool,
) {
    if force {
        std_out.queue(SetBackgroundColor(Color::Blue)).unwrap();
        std_out.queue(Clear(ClearType::All)).unwrap();
    }
    render_ui(std_out, score, force);
    for (x, col) in new_frame.iter().enumerate() {
        for (y, char) in col.iter().enumerate() {
            if *char != previous_frame[x][y] || force {
                std_out.queue(MoveTo(x as u16, (y + 1) as u16)).unwrap();
                print!("{}", *char)
            }
        }
    }
    std_out.flush().unwrap();
}

pub fn render_ui(std_out: &mut Stdout, score: u32, force: bool) {
    std_out.queue(SetBackgroundColor(Color::DarkCyan)).unwrap();
    if force {
        std_out.queue(MoveTo(0, 0)).unwrap();
        std_out.queue(Clear(ClearType::CurrentLine)).unwrap();
    }
    std_out.queue(MoveTo(0, 0)).unwrap();
    print!("Score: {}", score);
    std_out.queue(SetBackgroundColor(Color::Black)).unwrap();
}
