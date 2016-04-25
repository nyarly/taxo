use std::error::Error;
use taxo::Matchable;
use taxo::Result;

pub struct Rule {
  rule: glob::Pattern,
  opts: glob::MatchOptions,
  pub value: String,
}


extern crate glob;
impl Rule {
  pub fn new(rulestr: String, opts_opt: Option<String>, value: String) -> Result<super::Rule> {
    let pat = match glob::Pattern::new(&rulestr) {
      Err(err) => {
        return Err(format!("couldn't parse {} as a glob rule: {}",
                           rulestr,
                           err.description()))
      }
      Ok(p) => p,
    };

    let mut opts = glob::MatchOptions::new();
    opts.require_literal_leading_dot = true;

    match opts_opt {
      Some(optstr) => {
        for ch in optstr.chars() {
          match ch {
            'h' | 'd' => opts.require_literal_leading_dot = false,
            's' => opts.require_literal_separator = false,
            'S' => opts.require_literal_separator = true,
            'c' => opts.case_sensitive = true,
            'C' => opts.case_sensitive = false,
            unk => return Err(format!("Unrecognized glob rule option: {}", unk)),
          }
        }
      }
      None => {}
    }

    return Ok(super::Rule::Glob(Rule {
      rule: pat,
      opts: opts,
      value: value,
    }));
  }
}

impl Matchable for Rule {
  fn matches(&self, name: &str) -> bool {
    self.rule.matches_with(name, &self.opts)
  }
}
