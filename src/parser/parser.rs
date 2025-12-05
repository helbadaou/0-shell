
use crate::lexer;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
pub fn parse_tokens(tokens: Vec<lexer::tokenizer::Lexertype>) {
    let mut token_iter = tokens.into_iter();

    let command_name = match token_iter.next() {
        Some(lexer::tokenizer::Lexertype::Word(s)) => s,
        _ => {
            eprintln!("Error: Command must start with a command name.");
            return;
        }
    };

    let args: Vec<String> = token_iter
        .map(|token| {
            match token {
                lexer::tokenizer::Lexertype::Word(s) => s,
                lexer::tokenizer::Lexertype::Flag(s) => s,
                lexer::tokenizer::Lexertype::DoubleQuotedString(s) => s,
                lexer::tokenizer::Lexertype::SingleQuotedString(s) => s,
            }
        })
        .collect();

    let hh = execute_command(Command { name: command_name, args });
    if !hh {
        std::process::exit(0);
    }
}



pub fn execute_command(command: Command) -> bool {
    match command.name.as_str() {
        "exit" => {
            println!("Exiting the shell. Goodbye!");
            return false;
        }

        "help" => {
            println!("--- My Shell Help ---");
            println!("Built-in commands:");
            println!("  echo, cd, ls, pwd, cat, cp, rm, mv, mkdir, exit, help");
            println!("---------------------");
        }

        "echo" => {
            println!("{}", command.args.join(" "));
        }

        "pwd" => {
         
        }

        "cd" => {
          
        }

        "ls" => {
           
        }

        "cat" => {
          
        }

        "mkdir" => {
          
        }

        "rm" => {
        }

        "cp" => {
            
        }

        "mv" => {
        }

        _ => {
            eprintln!("Command not found: {}", command.name);
        }
    }

    true
}
