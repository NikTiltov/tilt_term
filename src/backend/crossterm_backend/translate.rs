use crate::{event::*, CellMods, Color};
use crossterm::{
  event::{self as ct},
  style::{Attribute, Attributes, Color as ctColor},
};

impl TryFrom<ct::Event> for Event {
  type Error = ();

  fn try_from(event: ct::Event) -> Result<Self, Self::Error> {
    match event {
      ct::Event::Key(key) => Ok(Event::Key(key.try_into()?)),
      ct::Event::Mouse(mouse) => Ok(Event::Mouse(mouse.try_into()?)),
      ct::Event::Resize(col, row) => Ok(Event::Resize(col as usize, row as usize)),
      _ => Err(()),
    }
  }
}

impl TryFrom<ct::KeyEvent> for KeyEvent {
  type Error = ();

  fn try_from(event: ct::KeyEvent) -> Result<Self, Self::Error> {
    Ok(Self {
      code: event.code.try_into()?,
      mods: event.modifiers.try_into()?,
    })
  }
}

impl TryFrom<ct::KeyCode> for KeyCode {
  type Error = ();

  fn try_from(code: ct::KeyCode) -> Result<Self, Self::Error> {
    match code {
      ct::KeyCode::Backspace => Ok(KeyCode::Backspace),
      ct::KeyCode::Enter => Ok(KeyCode::Enter),
      ct::KeyCode::Left => Ok(KeyCode::Left),
      ct::KeyCode::Right => Ok(KeyCode::Right),
      ct::KeyCode::Up => Ok(KeyCode::Up),
      ct::KeyCode::Down => Ok(KeyCode::Down),
      ct::KeyCode::Home => Ok(KeyCode::Home),
      ct::KeyCode::End => Ok(KeyCode::End),
      ct::KeyCode::PageUp => Ok(KeyCode::PageUp),
      ct::KeyCode::PageDown => Ok(KeyCode::PageDown),
      ct::KeyCode::Tab => Ok(KeyCode::Tab),
      ct::KeyCode::BackTab => Ok(KeyCode::Backtab),
      ct::KeyCode::Delete => Ok(KeyCode::Delete),
      ct::KeyCode::Insert => Ok(KeyCode::Insert),
      ct::KeyCode::F(n) => Ok(KeyCode::F(n)),
      ct::KeyCode::Char(ch) => Ok(KeyCode::Char(ch)),
      ct::KeyCode::Esc => Ok(KeyCode::Esc),
      _ => Err(()),
    }
  }
}

impl TryFrom<ct::KeyModifiers> for KeyMods {
  type Error = ();

  fn try_from(mods: ct::KeyModifiers) -> Result<Self, Self::Error> {
    Ok(KeyMods::from_bits(mods.bits() & KeyMods::ALL.bits()))
  }
}

impl TryFrom<ct::MouseEvent> for MouseEvent {
  type Error = ();

  fn try_from(event: ct::MouseEvent) -> Result<Self, Self::Error> {
    Ok(MouseEvent {
      code: event.kind.try_into()?,
      mods: event.modifiers.try_into()?,
      col: event.column as usize,
      row: event.row as usize,
    })
  }
}

impl TryFrom<ct::MouseEventKind> for MouseCode {
  type Error = ();

  fn try_from(kind: ct::MouseEventKind) -> Result<Self, Self::Error> {
    match kind {
      ct::MouseEventKind::Down(btn) => Ok(MouseCode::Down(btn.try_into()?)),
      ct::MouseEventKind::Up(btn) => Ok(MouseCode::Up(btn.try_into()?)),
      ct::MouseEventKind::Drag(btn) => Ok(MouseCode::Drag(btn.try_into()?)),
      ct::MouseEventKind::ScrollDown => Ok(MouseCode::ScrollDown),
      ct::MouseEventKind::ScrollUp => Ok(MouseCode::ScrollUp),
      _ => Err(()),
    }
  }
}

impl TryFrom<ct::MouseButton> for MouseButton {
  type Error = ();

  fn try_from(button: ct::MouseButton) -> Result<Self, Self::Error> {
    match button {
      ct::MouseButton::Left => Ok(MouseButton::Left),
      ct::MouseButton::Right => Ok(MouseButton::Right),
      ct::MouseButton::Middle => Ok(MouseButton::Middle),
    }
  }
}

impl From<Color> for ctColor {
  fn from(color: Color) -> Self {
    match color {
      Color::Rgb { r, g, b } => ctColor::Rgb { r, g, b },
      Color::Reset => ctColor::Reset,
      Color::Black => ctColor::Black,
      Color::Red => ctColor::DarkRed,
      Color::Green => ctColor::DarkGreen,
      Color::Yellow => ctColor::DarkYellow,
      Color::Blue => ctColor::DarkBlue,
      Color::Magenta => ctColor::DarkMagenta,
      Color::Cyan => ctColor::DarkCyan,
      Color::White => ctColor::Grey,
      Color::BrightBlack => ctColor::DarkGrey,
      Color::BrightRed => ctColor::Red,
      Color::BrightGreen => ctColor::Green,
      Color::BrightYellow => ctColor::Yellow,
      Color::BrightBlue => ctColor::Blue,
      Color::BrightMagenta => ctColor::Magenta,
      Color::BrightCyan => ctColor::Cyan,
      Color::BrightWhite => ctColor::White,
    }
  }
}

impl From<CellMods> for Attributes {
  fn from(mods: CellMods) -> Self {
    let mut attrs = Attributes::default();
    if mods & CellMods::BOLD != CellMods::NONE {
      attrs.set(Attribute::Bold);
    }
    if mods & CellMods::ITALIC != CellMods::NONE {
      attrs.set(Attribute::Italic);
    }
    if mods & CellMods::UNDERLINED != CellMods::NONE {
      attrs.set(Attribute::Underlined);
    }
    if mods & CellMods::UNDERCURLED != CellMods::NONE {
      attrs.set(Attribute::Undercurled);
    }
    if mods & CellMods::UNDERDOTTED != CellMods::NONE {
      attrs.set(Attribute::Bold);
    }
    if mods & CellMods::CROSSEDOUT != CellMods::NONE {
      attrs.set(Attribute::CrossedOut);
    }
    if mods & CellMods::REVERSE != CellMods::NONE {
      attrs.set(Attribute::Reverse);
    }
    attrs
  }
}
