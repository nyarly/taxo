use std::result::Result;

#[cfg(test)]mod test {
  #[test]
  fn it_works() {}
}

extern crate glob;
extern crate regex;
mod taxo {

  use std::io::BufReader;
  use std::io::BufRead;
  use std::fs::File;

  struct Rules {
    rules_list: Vec<Rule>,
  }

  enum Rule {
    Glob(GlobRule),
    Regex(RegexRule),
  }

  struct GlobRule {
    rule: glob::Pattern,
    opts: glob::MatchOptions,
    value: String,
  }

  struct RegexRule {
    rule: regex::Regex,
    value: String,
  }
  use glob;
  use std::error::Error;

  impl GlobRule {
    fn new(rulestr: String, opts_opt: Option<String>, value: String) -> Result<Rule, String> {
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

      return Ok(Rule::Glob(GlobRule {
        rule: pat,
        opts: opts,
        value: value,
      }));
    }
  }

  impl RegexRule {
    fn new(mut rulestr: String, options: Option<String>, value: String) -> Result<Rule, String> {
      let re: &regex::Regex;

      if let Some(optstr) = options {
        rulestr = format!("?({}){}", optstr, rulestr)
      };

      let re = match regex::Regex::new(&rulestr) {
        Err(_) => return Err(format!("Couldn't format '{}' as a regex", rulestr)),
        Ok(re) => re,
      };

      Ok(Rule::Regex(RegexRule {
        rule: re,
        value: value,
      }))
    }
  }


  use std::ops::Deref;
  impl Rule {
    fn parse(line: String) -> Result<Rule, String> {
      let mut parts = match line.chars().nth(1) {
        Some(ch) => line.split(ch).map(|s| String::from(s)),
        None => {
          return Err(format!("Rule '{}' is too short - try something like 'g * good'",
                             line))
        }
      };

      let kind = match parts.next() {
        Some(s) => match s.deref() {
          "g" | "G" | "f" | "F" => 'g',
          "r" | "R" => 'r',
          k => return Err(format!("unrecognized rule kind {} in {}", k, line)),
        },
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
  }

  trait Matchable {
    fn matches(&self, &str) -> bool;
  }

  impl Matchable for GlobRule {
    fn matches(&self, name: &str) -> bool {
      self.rule.matches_with(name, &self.opts)
    }
  }

  use regex;

  impl Rules {
    fn parse(source_file: &str) -> Result<Rules, String> {
      let mut rules_list = vec![];

      let f = match File::open("foo.txt") {
        Ok(file) => file,
        err @ Err(_) => return Err(format!("couldn't open file")),
      };
      let mut reader = BufReader::new(f);

      let rules = reader.lines().map(|line| {
        match line {
          Ok(l) => Rule::parse(String::from(l)),
          Err(e) => return Err(format!("io error while reading file {}", e.description())),
        }
      });

      for ruleRes in rules {
        match ruleRes {
          Err(e) => return Err(format!("parse error: {}", e)),
          Ok(rule) => rules_list.push(rule),
        }
      }

      Ok(Rules { rules_list: rules_list })
    }
  }
}
