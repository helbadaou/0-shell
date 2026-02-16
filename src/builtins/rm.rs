use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) {
    if args.is_empty() {
        eprintln!("rm: missing operand\r");
        return;
    }

    let mut recursive = false;
    let mut index = 0;
    while index < args.len() && args[index] == "-r" {
        recursive = true;
        index += 1;
    }

    let paths = &args[index..];
    if paths.is_empty() {
        eprintln!("rm: missing operand\r");
        return;
    }

    for path_str in paths {
        let path = Path::new(path_str);
        let metadata = match fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}\r", path_str, e);
                continue;
            }
        };

        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}\r", path_str, e);
            }
        } else if file_type.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}\r", path_str, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory\r", path_str);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}\r", path_str, e);
            }
        }
    }
}
