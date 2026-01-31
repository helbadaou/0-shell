use std::fs;
pub fn cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Error: to file at laste");
        return;
    };
    let src = &args[0];
    let dst = &args[1];

    match fs::copy(src, dst) {
        Ok(_) => println!("Copied {} -> {}", src, dst),
        Err(e) => eprintln!("Error: {}", e),
    }
}
