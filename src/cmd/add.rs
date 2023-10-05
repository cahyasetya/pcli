use clap::{Command, arg, ArgMatches};

pub fn get_command()  -> clap::Command {
  Command::new("add")
    .about("Adds files to myapp")
    .arg(arg!([NUM1]))
    .arg(arg!([NUM2]))
}

pub fn call(args: &ArgMatches) {
  let num1 = args.get_one::<String>("NUM1").unwrap().parse::<i32>().unwrap();
  let num2 = args.get_one::<String>("NUM2").unwrap().parse::<i32>().unwrap();
  println!(
    "Result: {:?}",
    num1 + num2
  )
}
