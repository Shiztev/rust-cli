/// Small CLI application.
/// 
/// Author: Stevie Alvarez

use std::env;

use cli::commands::command_selector;

mod cli;

fn main() {
  // get command line args
  let args: Vec<String> = env::args().collect();

  // loop on user input
  if args.len() == 1 {
    cli::run();

  } else {  // handle command line args
    command_selector(args[1].as_str(), args[2..].iter().map(AsRef::as_ref).collect::<Vec<&str>>().join(" ").as_str());
  }
}
