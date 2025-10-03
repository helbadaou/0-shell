use std::io::{self, Write};
use minishel::parser;
use minishel::excutor;
use minishel::builtins;

fn main() {
     let purple = "\x1b[35m";
    let cyan   = "\x1b[36m";
    let reset  = "\x1b[0m";
    let test = "\x1b[33m";

    println!("{}===================================={}", purple, reset);
    println!("{}       WELCOME TO 0-SHELL{}", cyan, reset);
    println!("{}   The minimal Rust command shell{}", cyan, reset);
    println!(r"{}
                        _              _   _  
  ___                  | |            | | | | 
 / _ \   ______   ___  | |__     ___  | | | | 
| | | | |______| / __| |  _ \   / _ \ | | | | 
| |_| |          \__ \ | | | | |  __/ | | | | 
 \___/           |___/ |_| |_|  \___| |_| |_| 
    {}", test, reset);
    println!("{}===================================={}", purple, reset);
    println!("Type 'help' to see built-in commands\n");
    loop {
        print!("~>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() { continue; }
        print!("{input}")
    }
}
