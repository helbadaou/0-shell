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
    SingleQuotedString, 
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


pub enum TokenizeResult {
    Success(Vec<Lexertype>),
    Error(String),
    Incomplete, 
}

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut tokens = Vec::new();
    let mut buffer = Buffer::new();
    let mut is_escaped = false; 

    for c in input.chars() {
        if is_escaped {
            buffer.buff.push(c);
            is_escaped = false;
            continue;
        }

        if c == '\\' {
            if buffer.typ != BufferType::SingleQuotedString {
                is_escaped = true;
                continue;
            }
        }

        match buffer.typ {
            BufferType::None => {
                if c == '\"' {
                    buffer.typ = BufferType::DoubleQuotedString;
                } else if c == '\'' {
                    buffer.typ = BufferType::SingleQuotedString; 
                } else if c == '-' {
                    buffer.typ = BufferType::Flag;
                    buffer.buff.push(c);
                } else if c.is_whitespace() {
                    continue;
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
                    buffer.typ = BufferType::SingleQuotedString;
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
                    buffer.typ = BufferType::SingleQuotedString;
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

            BufferType::SingleQuotedString => {
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


    if is_escaped {
        return TokenizeResult::Error("Syntax error: Trailing escape character \\".to_string());
    }

    match buffer.typ {
        BufferType::None => TokenizeResult::Success(tokens),
        BufferType::Word => {
            tokens.push(Lexertype::Word(buffer.buff));
            TokenizeResult::Success(tokens)
        }
        BufferType::Flag => {
            tokens.push(Lexertype::Flag(buffer.buff));
            TokenizeResult::Success(tokens)
        }
        BufferType::DoubleQuotedString | BufferType::SingleQuotedString => {
            TokenizeResult::Incomplete
        }
    }
}