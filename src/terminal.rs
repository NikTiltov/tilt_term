pub struct Terminal<B: crate::Backend>(B);

impl<B: crate::Backend> Terminal<B> {
  pub fn new() -> Self {
    Self(B::new())
  }
}

impl<B: crate::Backend> std::ops::Deref for Terminal<B> {
  type Target = B;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<B: crate::Backend> std::ops::DerefMut for Terminal<B> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
