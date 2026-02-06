use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let mut recursive = false;
    let mut index = 0;
    while index < args.len() && args[index] == "-r" {
        //mzl khas ntcheckee rh lflag khas ykun ri flbdya hit f pc dyali khas drr ykun flbdya
        recursive = true;
        index += 1;
    }

    let paths = &args[index..];
    if paths.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for path_str in paths {
        let path = Path::new(path_str);
        let metadata = match fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", path_str, e);
                continue;
            }
        };

        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            // Always remove symlink itself
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", path_str, e);
            }
        } else if file_type.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}", path_str, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path_str);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", path_str, e);
            }
        }
    }
}
