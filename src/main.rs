/// Small CLI application.
/// 
/// Author: Stevie Alvarez

use std::env;

mod cli;

fn main() {
  // get command line args
  let args: Vec<String> = env::args().collect();
  let l: usize = args.len();

  // loop on user input
  if l == 1 {
    cli::run();

  } else {  // handle command line args
    cli::command_selector(&args[1]);
  }
}
