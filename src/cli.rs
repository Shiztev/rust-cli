/// Command line prompt

pub fn run() {
  println!("cli run called");
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
