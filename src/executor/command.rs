use crate::lexer::Lexertype; // Make sure this path is correct

// 1. Removed the generic <B>
pub fn execute(tokens: Vec<Lexertype>) {
    // The closure now correctly returns Option<String>
    let mut words = tokens.into_iter().filter_map(|t| -> Option<String> {
        match t {
            // 2. KEEP the Word variant and extract its inner String
            Lexertype::Word(s) => Some(s),

            // 3. DISCARD all other variants by returning None
            Lexertype::Flag(_) |
            Lexertype::DoubleQuotedString(_) |
            Lexertype::SingleQuotedString(_) => None,
            
            // It's good practice to handle all variants you might have
            // e.g., Lexertype::Pipe => None,
        }
    });

    if let Some(command) = words.next() {
        // Now 'command' is guaranteed to be a String, so .as_str() works
        match command.as_str() {
            "exit" => {
                println!("Exiting 0-shell.");
                std::process::exit(0);
            }
            _ => {
                // You can get the rest of the arguments like this:
                let args: Vec<String> = words.collect();
                println!("Command not found: {}", command);
                println!("Arguments: {:?}", args); // Helpful for debugging
            }
        }
    }
}