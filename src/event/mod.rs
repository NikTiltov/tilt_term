mod key_code;
mod key_event;
mod key_mods;
mod mouse_button;
mod mouse_code;
mod mouse_event;

pub use self::{
  key_code::KeyCode, key_event::KeyEvent, key_mods::KeyMods, mouse_button::MouseButton,
  mouse_code::MouseCode, mouse_event::MouseEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
  Resize(usize, usize),
  Mouse(MouseEvent),
  Key(KeyEvent),
}
