use docopt::Docopt;
use taxo::Rules;

const USAGE: &'static str = "
Usage: taxo <rulepath> <filename>
";

#[derive(RustcDecodable)]
struct Args {
  arg_rulepath: str,
  arg_filename: str,
}

fn main() {
  let args: Args = DocOpt::new(USAGE)
    .and_then(|d| d.parse())
    .unwrap_or_else(|er| er.exit());

  let rules: Rules = Rules::parse(args.arg_rulepath);
  println!("{}", rules.matched_value(args.arg_filename))
}
