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

    // get command and args separatly
    let mut split = buf.splitn(2, " ");

    // run respective command
    commands::command_selector(split.next().unwrap(), split.next().unwrap());
    buf.clear();
  }
}

/// Collection of commands, which can be called with a public facing command selection function.
pub mod commands {
  use std::{process::{exit, Command}, fs::{File, OpenOptions}, io::{self, Write}};

  /// Run the specified command
  pub fn command_selector(cmd: &str, args: &str) {
    
    // if no command, exit
    if args.len() == 0 {
      return;
    }

    match cmd {
      "quit" | "exit" => exit(0),
      "help" => help(),
      "hello" => hello_world(),
      "write" => write_to_file(args),
      _ => exec_bash_cmd(cmd, args)
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
  /// modern cli's. 
  fn write_to_file(args: &str) {
    let a: Vec<&str> = args.splitn(3, "\"").collect::<Vec<&str>>();

    
    // basecase
    if args.len() < 3 {
      println!("Usage: write <filename> <\"text\">OR write <\"text\"> [Mode: >, >>] <filename>");
      return;
    }

    // open file
    let buf: String;
    let r: Result<File, io::Error>;
    let mut file: File;

    if a[2].contains(">") {
      let tmp = a[2].trim().splitn(2, " ").collect::<Vec<&str>>();
      r = OpenOptions::new().write(tmp[0] == ">").append(tmp[0] == ">>").create(true).open(tmp[1]);
      buf = a[1].to_string();

    } else {
      r = OpenOptions::new().write(true).create(true).open(a[0].trim());
      buf = a[1].to_string();
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

  /// Execute the command in the systems default cli
  fn exec_bash_cmd(cmd: &str, args: &str) {
    let mut c: Command = Command::new(cmd);
    let a = args.split_whitespace();
    
    // add args
    c.args(a);

    let r = c.spawn();
    
    match r {
      Ok(mut child) => {child.wait();},
      Err(er) => eprintln!("{}", er)
    }
  }
}
