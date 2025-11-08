use std::io::{ self, Write };
use minishel::lexer;
use minishel::parser::parser::parsing;
fn main() {
    let purple = "\x1b[35m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    println!("{}===================================={}", purple, reset);
    println!("{}       WELCOME TO 0-SHELL{}", cyan, reset);
    println!("{}   The minimal Rust command shell{}", cyan, reset);
    println!(
        r"
                                         _              _   _ 
                   ___                  | |            | | | | 
                  / _ \   ______   ___  | |__     ___  | | | | 
                 | | | | |______| / __| |  _ \   / _ \ | | | | 
                 | |_| |          \__ \ | | | | |  __/ | | | | 
                  \___/           |___/ |_| |_|  \___| |_| |_| 
    "
    );
    println!("{}======== f============================{}", purple, reset);
    println!("Type 'help' to see built-in commands\n");
    loop {
        print!("~>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let tokens: Vec<lexer::tokenizer::Lexertype> = lexer::tokenizer::tokenize(input);
        //executor::command::execute(tokens);
        parsing(tokens);
    }
}
