use std::fs;
pub fn mkdir(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        eprintln!("Usage: mkdir <directory>");
        return;
    }

    for dir in args {
        if let Err(e) = fs::create_dir(dir) {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
}