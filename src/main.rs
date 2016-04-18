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

  match taxo::parse_rulefile(args.arg_rulepath) {
    Ok(rules) => println!("{}", rules.matched_value(args.arg_filename).expect("")),
    Err(desc) => println!("{}", desc),
  }
}
