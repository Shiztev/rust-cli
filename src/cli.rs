/// Command line prompt

use std::io::{stdin, stdout, Write};

pub fn run() {
  let mut buf: String = String::new();

  // prompt user, handle output errors
  print!("r| ");
  stdout().flush().expect("Could not flush out command prompt");

  // get input, handle errors
  stdin().read_line(&mut buf).expect("failed to read line");

  // run respective command
  command_selector(&buf);



  println!("you entered: {}", buf);
}

/// Run the specified command
pub fn command_selector(c: &String) {

  match c.trim() {
    "help" => help(),
    "hello" => hello_world(),
    _ => println!("Usage: rust-cli [OPTIONS...]\nTry: 'rust-cli --help' for more information.")
  }
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
