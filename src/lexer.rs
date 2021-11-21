use anyhow::Result;

/// the types of tokens that are valid in the Lox language
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

/// A token is a single lexical unit of an input to the Lox Interpreter.
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // the line of the file that was parsed that this token was found on
    pub line: u32,
}

/// A lexer (or scanner) is responsible for breaking a program into a sequence of tokens.
pub struct Lexer {
    line: u32,
    // the start of the current lexeme being scanned
    start: u32,
    // the current character of the current lexeme being scanned
    end: u32,
}

/// the lexer is responsible for breaking an input program into a sequence of tokens. The program is represented 
/// as a string of characters which adhere to the Lox language syntax. 
impl Lexer {
    pub fn new() -> Self {
        Self {
            line: 1,
            start: 0,
            end: 0,
        }
    }

    // break a stream of bytes into a sequence of tokens.
    pub fn lex(&mut self, input: &str) -> Result<Vec<Token>> {
        
        let out  = input.lines().enumerate().map(|(line_number, line)| {
            self.lex_line(line, 1 + line_number as u32)
        }).collect::<Result<Vec<_>>>()?;

        Ok(out.into_iter().flatten().collect())
    }

    /// whether or not the lexer has reached the end of file
    fn is_at_end(&self, input : &str) -> bool {
        self.end >= input.len() as u32
    }

    fn lex_line(&self, input: &str , line : u32) -> Result<Vec<Token>> {
        todo!()
    }        
}

#[cfg(test)]
mod test {

    #[test]
    fn lexer_error_test() { 
        todo!()
    }

    #[test]
    fn lexer_lex_line_proper_test() { 
        todo!()
    }

    #[test]
    fn lexer_lex_line_error_test() { 
        todo!()
    }

    #[test]
    fn comment_test() { 
        todo!()
    }
}
