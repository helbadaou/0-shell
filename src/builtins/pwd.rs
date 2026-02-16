use crossterm::{ cursor, execute };
use std::env;
use std::io::{ self };
pub fn pwd() {
    execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(_) => {
            let cwd = crate::CWD.lock().unwrap();
            println!("{}", cwd);
        }
    }
}
