/// Command line prompt

use std::io::{stdin, stdout, Write};

pub fn run() {
  let mut buf: String = String::new();

  loop {
    // prompt user, handle output errors
    print!("r| ");
    stdout().flush().expect("Could not flush out command prompt");

    // get input, handle errors
    stdin().read_line(&mut buf).expect("failed to read line");

    // run respective command
    commands::command_selector(buf.trim().split(" ").collect());
    buf.clear();
  }
}

pub mod commands {
    use std::process::exit;

  /// Run the specified command
  pub fn command_selector(args: Vec<&str>) {
    
    // if no command, exit
    if args.len() == 0 {
      return;
    }

    match args[0] {
      "quit" | "exit" => exit(0),
      "help" => help(),
      "hello" => hello_world(),
      _ => println!("rust-cli: no command '{}'\nTry: 'rust-cli --help' for more information.", args[0])
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

  /// Write the provided text to the desired file.
  fn write_to_file() {

  }
}
