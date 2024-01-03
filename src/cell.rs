use crate::Color;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
  pub ch: char,
  pub fg: Color,
  pub bg: Color,
  pub mods: CellMods,
}

impl Default for Cell {
  fn default() -> Self {
    Self {
      ch: ' ',
      fg: Color::Reset,
      bg: Color::Reset,
      mods: CellMods::NONE,
    }
  }
}

impl From<char> for Cell {
  fn from(ch: char) -> Self {
    Self {
      ch,
      ..Default::default()
    }
  }
}

impl From<Color> for Cell {
  fn from(value: Color) -> Self {
    if value == Color::Black {
      Self::from('s')
    } else {
      Self::from('a')
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct CellMods(u8);

impl CellMods {
  pub const NONE: Self = Self(0 << 0);
  pub const BOLD: Self = Self(1 << 0);
  pub const ITALIC: Self = Self(1 << 1);
  pub const UNDERLINED: Self = Self(1 << 2);
  pub const UNDERCURLED: Self = Self(1 << 3);
  pub const UNDERDOTTED: Self = Self(1 << 4);
  pub const CROSSEDOUT: Self = Self(1 << 5);
  pub const REVERSE: Self = Self(1 << 6);
}

impl std::ops::BitOr for CellMods {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}

impl std::ops::BitOrAssign for CellMods {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl std::ops::BitAnd for CellMods {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}

impl std::ops::BitAndAssign for CellMods {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0;
  }
}
