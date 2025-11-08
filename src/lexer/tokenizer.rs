
#[derive(Debug, PartialEq, Clone)]
pub enum Lexertype {
    Word(String),
    Flag(String),
    DoubleQuotedString(String),
    SingleQuotedString(String), 
}

#[derive(Debug, PartialEq, Clone)]
pub enum BufferType {
    Word,
    Flag,
    DoubleQuotedString,
    QuotedString, 
    None,
}

pub struct Buffer {
    pub buff: String,
    pub typ: BufferType,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buff: String::new(),
            typ: BufferType::None,
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Lexertype> {
    let mut tokens = Vec::new();
    let mut buffer = Buffer::new();

    for c in input.chars() {
        match buffer.typ {
            BufferType::None => {
                if c == '\"' {
                    buffer.typ = BufferType::DoubleQuotedString;
                } else if c == '\'' {
                    buffer.typ = BufferType::QuotedString;
                } else if c == '-' { 
                    buffer.typ = BufferType::Flag;
                    buffer.buff.push(c);
                } else if c.is_whitespace() {
                    
                } else {
                    buffer.typ = BufferType::Word;
                    buffer.buff.push(c);
                }
            }

            BufferType::Word => {
                if c.is_whitespace() {
                    tokens.push(Lexertype::Word(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::None;
                } else if c == '\"' {
                    tokens.push(Lexertype::Word(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::DoubleQuotedString;
                } else if c == '\'' {
                    tokens.push(Lexertype::Word(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::QuotedString;
                } else if c == '-' { 
                    tokens.push(Lexertype::Word(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::Flag;
                    buffer.buff.push(c);
                } else {
                    buffer.buff.push(c);
                }
            }
            
            
            BufferType::Flag => {
                if c.is_whitespace() {
                    tokens.push(Lexertype::Flag(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::None;
                } else if c == '\"' {
                    tokens.push(Lexertype::Flag(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::DoubleQuotedString;
                } else if c == '\'' {
                    tokens.push(Lexertype::Flag(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::QuotedString;
                } else {
                    
                    buffer.buff.push(c);
                }
            }

            BufferType::DoubleQuotedString => {
                if c == '\"' {
                    tokens.push(Lexertype::DoubleQuotedString(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::None;
                } else {
                    buffer.buff.push(c);
                }
            }

            BufferType::QuotedString => {
                if c == '\'' {
                    
                    tokens.push(Lexertype::SingleQuotedString(buffer.buff.clone()));
                    buffer.buff.clear();
                    buffer.typ = BufferType::None;
                } else {
                    buffer.buff.push(c);
                }
            }
        }
    }

    
    if !buffer.buff.is_empty() {
        match buffer.typ {
            BufferType::Word => tokens.push(Lexertype::Word(buffer.buff)),
            BufferType::DoubleQuotedString => {
                tokens.push(Lexertype::DoubleQuotedString(buffer.buff))
            }
            BufferType::QuotedString => tokens.push(Lexertype::SingleQuotedString(buffer.buff)),
            BufferType::Flag => tokens.push(Lexertype::Flag(buffer.buff)), 
            BufferType::None => {}
        }
    }
        println!("{:?}", tokens);


    tokens
}

