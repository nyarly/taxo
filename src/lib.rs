use std::result::Result;

#[cfg(test)] mod test {
    #[test]
    fn it_works() {
    }
}

extern crate glob;
extern crate regex;
mod taxo {

  use std::fs::File;
  use std::io::BufReader;

  struct Rules {
    rules_list: Vec<Rule>
  }

  enum Rule {
    GlobRule,
    RegexpRule,
  }

  fn return_rule() -> Rule {
    GlobRule{value: "something", rule: glob::Pattern::new("").unwrap(), opts: glob::MatchOptions::new()} as Rule
  }

  impl Rule {
    fn parse(line: String) -> Result<Rule, String> {
      let chars = line.chars();
      let sep = chars.peekable().nth(1);

      let parts = match sep {
        Some(ch) => line.split(ch),
        None => return Err(format!("Rule '{}' is too short - try something like 'g * good'", line))
      };

      let kind = match parts.next() {
        Some("g") | Some("G") | Some("f") | Some("F") => "g",
        Some("r") | Some("R")                         => "r",
        None                                          => return Err(format!("Rule '{}' too short - no kind (rRgGfF)", line)),
        Some(k)                                       => return Err(format!("unrecognized rule kind {} in {}", k, line)),
      };

      let rule = match parts.next() {
        None => return Err(format!("Rule '{}' too short", line)),
        r @ Some(_) => r,
      };

      let value = match parts.next() {
        None => return Err(format!("Rule '{}' too short", line)),
        Some(v) => v,
      };

      match (kind, rule, parts.next()) {
        ("g" , Some(r)     , None)       => GlobRule::new(  r     , None , value) as Rule ,
        ("g" , r @ Some(_) , Some(last)) => GlobRule::new(  value , r    , last) as Rule ,
        ("r" , Some(r)     , None)       => RegexpRule::new(r     , None , value) as Rule,
        ("r" , r @ Some(_) , Some(last)) => RegexpRule::new(value , r    , last) as Rule ,
        _                 => Err(format!("Rule couldn't be parsed {}", line)),
      }

    }
  }

  trait Matchable {
    fn matches(&self, &str) -> bool;
  }

  use glob;

  struct GlobRule<'a> {
    rule: glob::Pattern,
    opts: glob::MatchOptions,
    value: &'a str,
  }

  impl<'a> GlobRule<'a> {
    fn new(rulestr: &str, opts_opt: Option<&str>, value: &str) -> Result<GlobRule<'a>,String> {
      let pat = match glob::Pattern::new(rulestr) {
        PatternErr => return Err(format!("couldn't parse {} as a glob rule", rulestr)),
        Ok(p) => p,
      };

      let opts = glob::MatchOptions::new();
      opts.require_literal_leading_dot = true;

      match opts_opt {
        Some(optstr) => for ch in optstr.chars() {
          match ch {
            "h" | "d" => opts.require_literal_leading_dot = false,
            "s" => opts.require_literal_separator = false,
            "S" => opts.require_literal_separator = true,
            "c" => opts.case_sensitive = true,
            "C" => opts.case_sensitive = false,
            unk => return Err(format!("Unrecognized glob rule option: {}", unk)),
          }
        },
        None => {}
      }

      return Ok(GlobRule{rule: pat, opts: opts, value: value})
    }
  }

  impl<'a> Matchable for GlobRule<'a> {
    fn matches(&self, name: &str) -> bool {
      self.rule.match_with(name, self.opts)
    }
  }

  use regex;

  struct RegexpRule<'a> {
    rule: &'a regex::Regex,
    value: &'a str,
  }

  impl<'a> RegexpRule<'a> {
    fn new(rulestr: &str, optstr: &str, value: &str) -> Result<RegexpRule<'a>,String> {
      if optstr.len() > 0 {
        rulestr = format!("?({}){}", optstr, rulestr)
      }
      let re = match regex::Regex.new(rulestr) {
        Err => return Err(format!("Couldn't format '{}' as a regex", rulestr)),
        Ok(re) => re,
      };

      Ok(RegexpRule{rule: re, value: value})
    }
  }

  impl Rules {
    fn parse(source_file: str) -> Result<Rules, &'static str> {
      let rules_list = vec![];

      let f = try!(File::open("foo.txt"));
      let mut reader = BufReader::new(f);

      let rules = reader.lines().map(|line| line.and_then(|l| Rule::parse(l)));
      if let err = rules.find(|r| r.not_ok()) {
        err
      } else {
        Ok(Rules{ rules_list: rules.collect() })
      }
    }
  }
}
