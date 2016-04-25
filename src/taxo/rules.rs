use std::error::Error;
use super::rule::{self, Rule};
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Result;
use super::Matchable;
use std::ops::Deref;

pub struct Rules {
  pub rules_list: Vec<Rule>,
}

pub fn parse_file(source_file: String) -> Result<Rules> {
  let f = match File::open(source_file) {
    Ok(file) => file,
    Err(err) => return Err(format!("couldn't open file: {}", err.description())),
  };
  let reader = BufReader::new(f);

  parse_buffer(reader)
}

pub fn parse_buffer<T: BufRead>(reader: T) -> Result<Rules> {
  let mut rules_list = vec![];

  let rules = reader.lines().map(|line| {
    match line {
      Ok(l) => if l != "" { rule::parse(String::from(l)).map(|r| Some(r)) } else { Ok(None) },
      Err(e) => return Err(format!("io error while reading file {}", e.description())),
    }
  });

  for rule_res in rules {
    match rule_res {
      Ok(None) => (),
      Ok(Some(rule)) => rules_list.push(rule),
      Err(e) => return Err(format!("parse error: {}", e)),
    }
  }

  Ok(Rules { rules_list: rules_list })
}

impl Rules {
  pub fn matched_value(&self, against: String) -> Option<&str> {
    self.rules_list
        .iter()
        .find(|&rule| {
          match rule {
            &Rule::Glob(ref gr) => gr.matches(&against),
            &Rule::Regex(ref rr) => rr.matches(&against),
          }
        })
        .map(|rule| {
          match rule {
            &Rule::Glob(ref gr) => gr.value.deref(),
            &Rule::Regex(ref rr) => rr.value.deref(),
          }
        })
  }
}
