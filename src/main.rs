use clap::Command;

pub mod cmd;
use cmd::add;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//   #[arg(short, long)]
//   name: String,

//   #[arg(short, long, default_value_t = 1)]
//   count: u8,
// }



fn main() {
  let matches = Command::new("pCli")
    .version("1.0")
    .author("Cahya S <cahya.setyaa@gmail.com>")
    .about("Personal Cli")
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      add::get_command()
    )
    .get_matches();

  match matches.subcommand() {
      Some(("add", sub_matches)) => {
        add::call(sub_matches);
      },
      _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
  }
}
