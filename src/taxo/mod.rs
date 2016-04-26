use std::result;

pub mod rules;
pub mod rule;

pub type Result<T> = result::Result<T, String>;

pub trait Matchable {
  fn matches(&self, &str) -> bool;
  fn value(&self) -> String;
}

impl<R: Matchable> Matchable for Box<R> {
  fn matches(&self, name: &str) -> bool {
    self.as_ref().matches(name)
  }

  fn value(&self) -> String {
    self.as_ref().value()
  }
}

mod test;
