use super::Result;

mod glob;
mod regex;

use self::glob::Rule as GlobRule;
use self::regex::Rule as RegexRule;

pub enum Rule {
  Glob(GlobRule),
  Regex(RegexRule),
}

use std::ops::Deref;
pub fn parse(line: String) -> Result<Rule> {
  let mut parts = match line.chars().nth(1) {
    Some(ch) => line.split(ch).map(|s| String::from(s)),
    None => {
      return Err(format!("Rule '{}' is too short - try something like 'g * good'",
                         line))
    }
  };

  let kind = match parts.next() {
    Some(s) => {
      match s.deref() {
        "g" | "G" | "f" | "F" => 'g',
        "r" | "R" => 'r',
        k => return Err(format!("unrecognized rule kind {} in {}", k, line)),
      }
    }
    None => return Err(format!("Rule '{}' too short - no kind (rRgGfF)", line)),
  };

  let rule = match parts.next() {
    None => return Err(format!("Rule '{}' too short", line)),
    Some(r) => r,
  };

  let value = match parts.next() {
    None => return Err(format!("Rule '{}' too short", line)),
    Some(v) => v,
  };

  match (kind, parts.next()) {
    ('g', None) => GlobRule::new(rule, None, value),
    ('g', Some(last)) => GlobRule::new(value, Some(rule), last),
    ('r', None) => RegexRule::new(String::from(rule), None, value),
    ('r', Some(last)) => RegexRule::new(String::from(value), Some(rule), last),
    _ => Err(format!("Rule couldn't be parsed {}", line)),
  }
}
