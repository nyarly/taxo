use std::result;

pub mod rules;
pub mod rule;

pub type Result<T> = result::Result<T, String>;

pub use self::rule::Rule;

pub trait Matchable {
  fn matches(&self, &str) -> bool;
}

mod test;
