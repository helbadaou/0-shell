use std::fs::File;
use std::io::{ self, Read, Write };
use std::path::Path;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };

pub fn cat(args: &[String]) {
    let mut stdout = io::stdout();

    if args.is_empty() {
        let _ = disable_raw_mode();

        let mut stdin = io::stdin();
        match io::copy(&mut stdin, &mut stdout) {
            Err(e) => {
                let error_msg = format!("0-shell: cat: {}\r\n", e);
                let _ = io::stderr().write_all(error_msg.as_bytes());
            }
            Ok(_) => {}
        }

        let _ = enable_raw_mode();
        return;
    }

    for filename in args {
        let path = Path::new(filename);

        if path.is_dir() {
            let error_msg = format!("0-shell: cat: {}: Is a directory\r\n", filename);
            let _ = io::stderr().write_all(error_msg.as_bytes());
            continue;
        }

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let output = contents.replace("\n", "\r\n");
                        let _ = io::stdout().write_all(output.as_bytes());
                    }
                    Err(e) => {
                        let error_msg = format!("0-shell: cat: {}: {}\r\n", filename, e);
                        let _ = io::stderr().write_all(error_msg.as_bytes());
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("0-shell: cat: {}: {}\r\n", filename, e);
                let _ = io::stderr().write_all(error_msg.as_bytes());
            }
        }
    }
}
