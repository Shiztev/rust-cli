/// Command line prompt

use std::io::{stdin, stdout, Write};

pub fn run() {
  let mut buf: String = String::new();

  println!("\nWelcome to my rust cli. Note that piping is not supported.");

  loop {
    // prompt user, handle output errors
    print!("r| ");
    stdout().flush().expect("Could not flush out command prompt");

    // get input, handle errors
    stdin().read_line(&mut buf).expect("failed to read line");

    // run respective command
    commands::command_selector(buf.trim().split_whitespace().collect());
    buf.clear();
  }
}

pub mod commands {
  use std::{process::{exit, Command}, vec::IntoIter, fs::{File, OpenOptions}, io::{self, Read, Write}};

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
      "write" => write_to_file(args),
      _ => exec_bash_cmd(args.into_iter())//println!("rust-cli: no command '{}'\nTry: 'rust-cli --help' for more information.", args[0])
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
  /// 
  /// This inherintly overwrites the standard 'write' command in most 
  /// modern clis. 
  fn write_to_file(args: Vec<&str>) {
    // basecase
    if args.len() < 3 {
      println!("Usage: write <filename> <text>");
      return;
    }

    // open file
    let buf: String;
    let r: Result<File, io::Error>;
    let mut file: File;

    if !args[2].contains(">") {
      r = OpenOptions::new().write(true).create(true).open(args[1]);
      buf = args[2].to_string();

    } else if args.len() == 4 {  // TODO Currently, split on whitespace -> only write one word
      r = OpenOptions::new().write(args[2] == ">").append(args[2] == ">>").create(true).open(args[3]);
      buf = args[1].to_string();

    } else {
      println!("Usage: write <filename> <text> OR write <text> [Mode: >, >>] <filename>");
      return;
    }

    // write to file
    match r {
      Ok(o) => {file = o},
      Err(er) => {eprintln!("{}", er); return;}
    }

    match file.write_all(buf.as_bytes()) {
      Ok(()) => {},
      Err(er) => {eprintln!("{}", er); return;}
    }
  }

  /// Execute the command in bash
  fn exec_bash_cmd(mut args: IntoIter<&str>) {
    let mut c: Command = Command::new(args.next().unwrap());

    // add args
    c.args(args);

    let r = c.spawn();
    
    match r {
      Ok(mut child) => {child.wait();},
      Err(er) => eprintln!("{}", er)
    }
  }
}
