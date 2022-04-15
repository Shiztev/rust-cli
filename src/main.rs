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
    if args[1] == "--help" {
      println!("Usage: rust-cli [OPTIONS...]\n\n--help\n\tprint out program help information
      \n\n--hello\n\tprint \"Hello World!\"");

    } else if args[1] == "--hello" {
      println!("Hello World!");

    } else {
      println!("Usage: rust-cli [OPTIONS...]\nTry: 'rust-cli --help' for more information.");
    }
  }
}
