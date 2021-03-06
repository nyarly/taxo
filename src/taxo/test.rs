#![cfg(test)]

use super::*;

#[test]
fn it_works() {}

#[test]
fn glob_parses() {
  assert_eq!("good",
             rule::parse("g **/*.rs good".to_string()).unwrap().value())
}

#[test]
fn glob_matches() {
  let glob = rule::parse("g **/yes.rs good".to_string()).unwrap();

  assert!(glob.matches("somewhere/yes.rs"));
  assert!(glob.matches("yes.rs"));
  assert!(glob.matches("somewhere/somewhen/yes.rs"));
  assert!(!glob.matches("somewhere/no.rs"))
}

#[test]
fn regex_matches() {
  let re = rule::parse("r .*yes.rs$ good".to_string()).unwrap();

  assert!(re.matches("somewhere/yes.rs"));
  assert!(re.matches("yes.rs"));
  assert!(re.matches("somewhere/somewhen/yes.rs"));
  assert!(!re.matches("somewhere/no.rs"))
}

#[test]
fn rule_matching() {
  let rule_str = "
g **/glob.rs globbed
r .*/regex.rs rexed
";
  let rules = super::rules::parse_buffer(rule_str.as_bytes()).unwrap();
  assert_eq!("globbed",
             rules.matched_value("somewhere/glob.rs".to_string()).expect("No matched value"));
  assert_eq!("rexed",
             rules.matched_value("somewhere/regex.rs".to_string()).expect("No matched value"));
  assert!(rules.matched_value("nothing".to_string()).is_none())
}
