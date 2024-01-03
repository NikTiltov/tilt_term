use crate::{KeyMods, MouseCode};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MouseEvent {
  pub code: MouseCode,
  pub mods: KeyMods,
  pub col: usize,
  pub row: usize,
}

impl MouseEvent {
  pub fn new(code: MouseCode, mods: KeyMods, col: usize, row: usize) -> Self {
    Self {
      code,
      mods,
      col,
      row,
    }
  }
}
