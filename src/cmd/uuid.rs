use clap::Command;
use uuid::Uuid;

pub fn get_command() -> clap::Command {
  Command::new("uuid")
    .about("Generate uuid")
}

pub fn call() {
  let uuid = Uuid::new_v4();
  println!("{}", uuid);
}
