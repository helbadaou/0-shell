use std::{ fs, path::{ Path, PathBuf } };
use std::io;

pub fn mv(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: mv <source> <destination>\r");
        return;
    }

    let dst = Path::new(&args[args.len() - 1]);

    // Check if destination exists and is a directory
    let dst_is_dir = dst.exists() && dst.is_dir();

    for arg in args[..args.len() - 1].iter() {
        let src = Path::new(arg);

        // Check if source exists
        if !src.exists() {
            eprintln!("mv: {}: No such file or directory\r", arg);
            continue;
        }

        let final_dst: PathBuf = if dst_is_dir {
            // Moving into a directory
            let name = src.file_name().expect("Invalid source");
            dst.join(name)
        } else {
            // Renaming to destination
            dst.to_path_buf()
        };

        match fs::rename(src, &final_dst) {
            Ok(_) => {}
            Err(e) => eprintln!("mv: cannot move '{}' to '{}': {}\r", arg, final_dst.display(), e),
        }
    }
}