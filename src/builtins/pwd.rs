use std::env;
use std::process;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            process::exit(1);
        }
    }
}