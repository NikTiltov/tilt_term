mod translate;

use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event, execute, queue,
  style::{Color, Print, SetBackgroundColor, SetForegroundColor},
  terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Write;

pub struct Crossterm {
  stdout: std::io::Stdout,
}

impl Crossterm {
  pub fn new() -> Self {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
    Self { stdout }
  }
}

impl Drop for Crossterm {
  fn drop(&mut self) {
    terminal::disable_raw_mode().unwrap();
    execute!(self.stdout, LeaveAlternateScreen, Show).unwrap();
  }
}

impl crate::Backend for Crossterm {
  fn size(&self) -> (usize, usize) {
    let (col, row) = terminal::size().unwrap();
    (col as usize, row as usize)
  }

  fn cursor_show(&mut self) {
    queue!(self.stdout, Show).unwrap();
  }

  fn cursor_hide(&mut self) {
    queue!(self.stdout, Hide).unwrap();
  }

  fn set_cursor(&mut self, col: usize, row: usize) {
    queue!(self.stdout, MoveTo(col as u16, row as u16)).unwrap();
  }

  fn event(&mut self) -> crate::Event {
    loop {
      if let Ok(event) = event::read() {
        if let Ok(event) = event.try_into() {
          return event;
        }
      }
    }
  }

  fn draw<'a>(&mut self, content: impl Iterator<Item = (usize, usize, &'a crate::Cell)>) {
    for (x, y, cell) in content {
      queue!(
        self.stdout,
        MoveTo(x as u16, y as u16),
        Print(cell.ch),
        SetForegroundColor(Color::from(cell.fg)),
        SetBackgroundColor(Color::from(cell.fg)),
      )
      .unwrap()
    }
  }

  fn clear(&mut self) {
    queue!(self.stdout, Clear(ClearType::All)).unwrap();
  }

  fn flush(&mut self) {
    self.stdout.flush().unwrap();
  }
}