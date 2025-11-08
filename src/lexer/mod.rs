pub mod tokenizer;
pub use Lexertype::*;
#[derive(Debug, PartialEq)]
pub enum Lexertype {
    Word(String),
    Flag(String),
    DoubleQuotedString(String),
    SingleQuotedString(String),
}
