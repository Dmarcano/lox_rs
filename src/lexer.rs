use anyhow::{Result};

pub struct Token; 

/// A lexer (or scanner) is responsible for breaking a program into a sequence of tokens.
pub struct Lexer; 

impl Lexer { 
    pub fn new() -> Self {
        Lexer
    }

    // break a stream of bytes into a sequence of tokens.
    pub fn lex(&self, input: &str) -> Result<Vec<Token>> {
        todo!()
    }
    
}