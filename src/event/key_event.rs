use super::{KeyCode, KeyMods};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
  pub code: KeyCode,
  pub mods: KeyMods,
}

impl KeyEvent {
  pub fn new(code: KeyCode, mods: KeyMods) -> Self {
    Self { code, mods }
  }

  pub fn code(code: KeyCode) -> Self {
    Self::new(code, KeyMods::NONE)
  }

  pub fn shift(code: KeyCode) -> Self {
    Self::new(code, KeyMods::SHIFT)
  }

  pub fn ctrl(code: KeyCode) -> Self {
    Self::new(code, KeyMods::CTRL)
  }

  pub fn alt(code: KeyCode) -> Self {
    Self::new(code, KeyMods::ALT)
  }
}
