use std::fs::File;
use std::io::{ self };
use std::path::Path;


//mzl "cat - file.txt" --> aykhasha tkhdm b controlD
pub fn cat(args: &[String]) {
    let mut stdout = io::stdout();

    if args.is_empty() {
        let mut stdin = io::stdin();
        match io::copy(&mut stdin, &mut stdout) {
            Err(e) => {
                eprintln!("0-shell: cat: {}", e);
            }
            Ok(_) => {}
        }
        return
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
