use crate::lexer;
use crossterm::terminal::disable_raw_mode;
use crossterm::{ cursor, execute };
use std::io::{ self };
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

    let hh = execute_command(Command {
        name: command_name,
        args,
    });
    if !hh {
        let _ = disable_raw_mode();
        std::process::exit(0);
    }
}

pub fn execute_command(command: Command) -> bool {
    match command.name.as_str() {
        "exit" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            println!("Exiting the shell. Goodbye!");
            return false;
        }

        "help" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            println!("--- My Shell Help ---\r");
            println!("Built-in commands:\r");
            println!("  echo, cd, ls, pwd, cat, cp, rm, mv, mkdir, exit, help\r");
            println!("---------------------\r");
        }

        "echo" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            println!("{}", command.args.join(" "));
        }

        "pwd" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();

            crate::builtins::pwd::pwd();
        }

        "cd" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();

            crate::builtins::cd::cd(&command.args);
        }

        "ls" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();

            crate::builtins::ls::ls(&command.args);
        }

        "cat" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();

            crate::builtins::cat::cat(&command.args);
        }

        "mkdir" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();

            crate::builtins::mkdir::mkdir(&command.args);
        }

        "rm" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            crate::builtins::rm::rm(&command.args);
        }

        "cp" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            crate::builtins::cp::cp(&command.args);
        }

        "mv" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            crate::builtins::mv::mv(&command.args);
        }
        "clear" => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            print!("\x1B[2J\x1B[H\x1B[3J");
        }
        _ => {
            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
            eprintln!("Command not found: {}", command.name);
        }
    }

    true
}
