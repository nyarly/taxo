use taxo::{Matchable, Result};

pub struct Rule {
  rule: regex::Regex,
  pub value: String,
}

extern crate regex;
impl Rule {
  pub fn new(mut rulestr: String, options: Option<String>, value: String) -> Result<Box<Rule>> {
    if let Some(optstr) = options {
      rulestr = format!("?({}){}", optstr, rulestr)
    };

    let re = match regex::Regex::new(&rulestr) {
      Err(_) => return Err(format!("Couldn't format '{}' as a regex", rulestr)),
      Ok(re) => re,
    };

    Ok(Box::new(Rule {
      rule: re,
      value: value,
    }))
  }
}

impl Matchable for Rule {
  fn matches(&self, name: &str) -> bool {
    self.rule.is_match(name)
  }

  fn value(&self) -> String {
    self.value.clone()
  }
}
