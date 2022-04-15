/// Command line prompt

use std::io::{stdin, stdout, Write};

pub fn run() {
  println!("cli run called");
  let r;
  let mut buf = String::new();

  print!("r| ");
  stdout().flush().expect("Could not flush out command prompt");
  r = stdin().read_line(&mut buf);

  match r {
    Ok(thing) => thing,
    Err(error) => panic!("{:?}", error)
  };

  println!("you entered: {}", buf);
}

/// Run the specified command
/// 
/// if command exists, run and return true
/// 
/// if command does not exist, return false
pub fn command_selector(c: &String) -> bool {
  if c == "help" {
    help();

  } else if c == "hello" {
    hello_world();

  } else {
    println!("Usage: rust-cli [OPTIONS...]\nTry: 'rust-cli --help' for more information.");
    return false;
  }

  true
}

/// Print CLI help
fn help() {
  println!("Usage: rust-cli [OPTIONS...]\n\nhelp\n\tprint out program help information
  \n\nhello\n\tprint \"Hello World!\"");
}

/// Print "Hello World!"
fn hello_world() {
  println!("Hello World!");
}
