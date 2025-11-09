use crate::lexer;

pub fn parsing(lexer:Vec<lexer::tokenizer::Lexertype>) {
    println!("Parsing function called with tokens: {:?}", lexer[0]);    
    if let lexer::tokenizer::Lexertype::Word(word_content) = &lexer[0] {
        println!("{}", word_content);
    }

}