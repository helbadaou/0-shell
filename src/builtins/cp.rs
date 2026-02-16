use crossterm::{ cursor, execute };
use std::io;
use std::{ fs, path::{ Path, PathBuf } };

pub fn cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: cp <source>... <destination>\r");
        return;
    }

    let dst = Path::new(&args[args.len() - 1]);
    if args.len() > 2 && !dst.is_dir() {
        eprintln!("cp: target '{}' is not a directory\r", dst.display());
        return;
    }

    for src_str in &args[..args.len() - 1] {
        let src = Path::new(src_str);

        if !src.exists() {
            eprintln!("cp: cannot stat '{}': No such file or directory\r", src.display());
            continue;
        }

        if src.is_dir() {
            eprintln!("cp: -r not specified; omitting directory '{}'\r", src.display());
            continue;
        }

        let final_dst: PathBuf = if dst.is_dir() {
            match src.file_name() {
                Some(name) => dst.join(name),
                None => {
                    eprintln!("cp: invalid path '{}'\r", src.display());
                    continue;
                }
            }
        } else {
            dst.to_path_buf()
        };

        match fs::copy(src, &final_dst) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("cp: cannot copy '{}' to '{}': {}\r", src.display(), final_dst.display(), e);
            }
        }
    }
}