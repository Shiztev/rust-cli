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
  if l == 0 {
    cli::run();

  } else {  // handle command line args
    if args[1] == "hello" {
      println!("Hello World!");
    }
  }
}
