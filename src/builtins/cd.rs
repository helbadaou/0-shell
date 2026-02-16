use std::env;
use std::path::Path;


pub fn cd(args: &[String]) {
    let path = if args.is_empty() {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME environment variable not set\r");
                return;
            }
        }
    } else {
        args[0].clone()
    };

    let new_dir = Path::new(&path);

    if let Err(e) = env::set_current_dir(new_dir) {
        eprintln!("cd: {}: {}\r", path, e);
    } else {
        if let Ok(canonical) = env::current_dir() {
            let mut cwd = crate::CWD.lock().unwrap();
            *cwd = canonical.to_string_lossy().to_string();
        }
    }
}