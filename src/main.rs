use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::{cursor, execute};
use minishel::lexer;
use minishel::parser::parser::parse_tokens;
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
    println!("{}========f============================{}", purple, reset);
    println!("Type 'help' to see built-in commands\n");
    let mut line_buffer = String::new();
    loop {
        if line_buffer.is_empty() {
            print!("$ ");
        } else {
            print!("> ");
        }
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!();
            break;
        }*/
        // === READ ONE COMMAND ===
        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        return Ok(());
                    }

                    // Ctrl+D: Exit the shell
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        println!();
                        disable_raw_mode()?;
                        return Ok(());
                    }

                    // Ctrl+C: Cancel current input
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        println!("^C");
                        input.clear();
                        line_buffer.clear();
                        break;
                    }

                    KeyCode::Enter => {
                        execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                        println!();
                        io::stdout().flush().unwrap();
                        break;
                    }

                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            print!("\u{8} \u{8}");
                            io::stdout().flush().unwrap();
                        }
                    }

                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }

                    _ => {} // arrows ignored
                }
            }
        }

        let input= input.trim();
        if input.is_empty() {
            continue;
        }
        if !line_buffer.is_empty() {
            line_buffer.push('\n');
        }
        line_buffer.push_str(input.trim_end());

        match lexer::tokenizer::tokenize(&line_buffer) {
            lexer::tokenizer::TokenizeResult::Success(tokens) => {
                println!("Got tokens: {:?}", tokens);
                parse_tokens(tokens);

                line_buffer.clear();
            }

            lexer::tokenizer::TokenizeResult::Incomplete => {
                continue;
            }

            lexer::tokenizer::TokenizeResult::Error(err) => {
                eprintln!("{}", err);
                line_buffer.clear();
            }
        }
    }
}

