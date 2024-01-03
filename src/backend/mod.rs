mod crossterm_backend;
pub use crossterm_backend::Crossterm;

pub trait Backend {
  fn size(&self) -> (usize, usize);
  fn cursor_show(&mut self);
  fn cursor_hide(&mut self);
  fn set_cursor(&mut self, col: usize, row: usize);
  fn event(&mut self) -> crate::Event;
  fn draw<'a>(&mut self, content: impl Iterator<Item = (usize, usize, &'a crate::Cell)>);
  fn clear(&mut self);
  fn flush(&mut self);
}
