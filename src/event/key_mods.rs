#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyMods(u8);

impl KeyMods {
  pub const NONE: Self = Self(0 << 0);
  pub const SHIFT: Self = Self(1 << 0);
  pub const CTRL: Self = Self(1 << 1);
  pub const ALT: Self = Self(1 << 2);
  pub const ALL: Self = Self(0b0111);

  pub const fn from_bits(bits: u8) -> Self {
    Self(bits)
  }

  pub fn bits(&self) -> u8 {
    self.0
  }
}

impl std::ops::BitOr for KeyMods {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}

impl std::ops::BitOrAssign for KeyMods {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl std::ops::BitAnd for KeyMods {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}

impl std::ops::BitAndAssign for KeyMods {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0;
  }
}
