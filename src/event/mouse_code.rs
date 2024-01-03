use crate::MouseButton;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MouseCode {
  Down(MouseButton),
  Up(MouseButton),
  Drag(MouseButton),
  ScrollDown,
  ScrollUp,
}
