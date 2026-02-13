use std::{fs, path::{Path, PathBuf}};
pub fn mv(args: &[String]) {
    if args.len() != 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);

    let final_dst: PathBuf = if dst.is_dir() {
        let name = src.file_name().expect("Invalid source");
        dst.join(name)
    } else {
        dst.to_path_buf()
    };

    match fs::rename(src, &final_dst) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
