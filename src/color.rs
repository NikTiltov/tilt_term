#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
  Rgb { r: u8, g: u8, b: u8 },
  Reset,
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
}

impl Color {
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::Rgb { r, g, b }
  }

  pub const fn hex(hex: u32) -> Self {
    let r = (hex / 0x10000) as u8;
    let g = (hex / 0x100 % 0x100) as u8;
    let b = (hex % 0x100) as u8;
    Self::rgb(r, g, b)
  }
}
