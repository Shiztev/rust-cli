/// Command line prompt

use std::{io::{stdin, stdout, Write}, process::exit};

pub fn run() {
  let mut buf: String = String::new();

  loop {
    // prompt user, handle output errors
    print!("r| ");
    stdout().flush().expect("Could not flush out command prompt");

    // get input, handle errors
    stdin().read_line(&mut buf).expect("failed to read line");

    // run respective command
    command_selector(buf.trim());
    buf.clear();
  }
}

/// Run the specified command
pub fn command_selector(c: &str) {

  match c {
    "quit"  => exit(0),
    "help" => help(),
    "hello" => hello_world(),
    _ => println!("rust-cli: no command '{}'\nTry: 'rust-cli --help' for more information.", c)
  }

  println!();
}

/// Print CLI help
fn help() {
  println!("Usage: rust-cli [OPTIONS...]
  \n\n\thelp\n\t\tprint out program help information
  \n\n\thello\n\t\tprint \"Hello World!\"");
}

/// Print "Hello World!"
fn hello_world() {
  println!("Hello World!");
}
