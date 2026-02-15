use std::sync::{LazyLock, Mutex};

pub mod builtins;
pub mod lexer;
pub mod parser;

pub static CWD: LazyLock<Mutex<String>> = LazyLock::new(|| {
    Mutex::new(
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| String::from("/")),
    )
});
