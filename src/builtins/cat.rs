use std::fs::File;
use std::io::{ self };
use std::path::Path;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };

pub fn cat(args: &[String]) {
    let mut stdout = io::stdout();

    if args.is_empty() {
        // Disable raw mode to allow normal stdin reading
        let _ = disable_raw_mode();
        
        let mut stdin = io::stdin();
        match io::copy(&mut stdin, &mut stdout) {
            Err(e) => {
                eprintln!("0-shell: cat: {}", e);
            }
            Ok(_) => {}
        }
        
        // Re-enable raw mode for the shell
        let _ = enable_raw_mode();
        return;
    }
    
    for filename in args {
        let path = Path::new(filename);
        match File::open(path) {
            Ok(mut file) => {
                match io::copy(&mut file, &mut stdout) {
                    Err(e) => {
                        eprintln!("0-shell: cat: {}: {}", filename, e);
                    }
                    Ok(_) => {}
                }
            }
            Err(e) => {
                eprintln!("0-shell: cat: {}: {}", filename, e);
            }
        }
    }
}