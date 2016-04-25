extern crate docopt;
extern crate rustc_serialize;

mod taxo;

use docopt::Docopt;

const USAGE: &'static str = "
Usage: taxo <rulepath> <filename>
";

#[derive(RustcDecodable)]
struct Args {
  arg_rulepath: String,
  arg_filename: String,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
                     .and_then(|d| d.decode())
                     .unwrap_or_else(|er| er.exit());

  match taxo::rules::parse_file(args.arg_rulepath) {
    Ok(rules) => {
      match rules.matched_value(args.arg_filename) {
        Some(value) => println!("{}", value),
        None => println!("<unknown>"),
      }
    }
    Err(desc) => println!("{}", desc),
  }
}
