use std::{iter::Peekable, str::Chars};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
/// A token is a single lexical unit of an input to the Lox Interpreter.
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // the line of the file that was parsed that this token was found on
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
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

    /// break a string-slice of utf8-characters into a sequence of tokens.
    pub fn lex(&mut self, input: &str) -> Result<Vec<Token>> {
        let out = input
            .lines()
            .enumerate()
            .map(|(line_number, line)| self.lex_line(line, 1 + line_number as u32))
            .collect::<Result<Vec<_>>>()?;

        Ok(out.into_iter().flatten().collect())
    }

    /// whether or not the lexer has reached the end of file
    fn is_at_end(&self, input: &str) -> bool {
        self.end >= input.len() as u32
    }

    fn lex_line(&self, line: &str, line_number: u32) -> Result<Vec<Token>> {
        // let tokens = line.split_whitespace()
        //     .map(|word| word.chars())
        //     .map(|chars| self.lex_chars(chars, line_number))
        //     .collect::<Result<Vec<_>>>()?;

        let tokens = self.lex_chars(line.chars(), line_number)?;
        Ok(tokens)
            // Ok(tokens.into_iter().flatten().collect())
    }

    // TODOOOO: Handle comments
    /// Handles lexing/scanning on a character by character basis. This way multi-character tokens can be either split into multiple smaller tokens or into a larger identifier token.
    /// 
    /// ### Note
    /// The lexer
    fn lex_chars(&self, word: Chars, line_number: u32) -> Result<Vec<Token>> {
        /*
        Use a Peekable iterator to allow us to peek at the next character in the input without consuming the iterator at the current character
        This is useful for determining whether or not a token is a multi-character token or a comment. 
        This is what is called single-character lookahead and is used by many parsing algorithms.
        */
        let mut peek: Peekable<_> = word.peekable();
        let mut tokens = Vec::new();

        // keep looping until we reach the end of the iterator
        while let Some(char) = peek.next() {
            let lexeme = char.to_string();
            let next_peek = peek.peek();

            let out = match &*lexeme {
                "(" => Ok(Token::new(TokenType::LeftParen, lexeme, line_number)),
                ")" => Ok(Token::new(TokenType::RightParen, lexeme, line_number)),
                "{" => Ok(Token::new(TokenType::LeftBrace, lexeme, line_number)),
                "}" => Ok(Token::new(TokenType::RightBrace, lexeme, line_number)),
                "," => Ok(Token::new(TokenType::Comma, lexeme, line_number)),
                "." => Ok(Token::new(TokenType::Dot, lexeme, line_number)),
                "-" => Ok(Token::new(TokenType::Minus, lexeme, line_number)),
                "+" => Ok(Token::new(TokenType::Plus, lexeme, line_number)),
                ";" => Ok(Token::new(TokenType::Semicolon, lexeme, line_number)),
                "*" => Ok(Token::new(TokenType::Star, lexeme, line_number)),
                "/" => {
                    if next_peek == Some(&'/') {
                        // ignore comments
                        while let Some(char) = peek.next() {
                            if char == '\n' {
                                break;
                            }
                        }
                        continue
                    } else {
                        Ok(Token::new(TokenType::Slash, lexeme, line_number))
                    }
                },
                "!" => {
                    if next_peek == Some(&'=') {
                        peek.next();
                        Ok(Token::new(TokenType::BangEqual, lexeme, line_number))
                    } else {
                        Ok(Token::new(TokenType::Bang, lexeme, line_number))
                    }
                }
                "=" => {
                    if next_peek == Some(&'=') {
                        peek.next();
                        Ok(Token::new(TokenType::EqualEqual, lexeme, line_number))
                    } else {
                        Ok(Token::new(TokenType::Equal, lexeme, line_number))
                    }
                }
                ">" => {
                    if next_peek == Some(&'=') {
                        peek.next();
                        Ok(Token::new(TokenType::GreaterEqual, lexeme, line_number))
                    } else {
                        Ok(Token::new(TokenType::Greater, lexeme, line_number))
                    }
                }
                "<" => {
                    if next_peek == Some(&'=') {
                        peek.next();
                        Ok(Token::new(TokenType::LessEqual, lexeme, line_number))
                    } else {
                        Ok(Token::new(TokenType::Less, lexeme, line_number))
                    }
                },
                " " | "\r" | "\t" => {
                    // ignore whitespace characters
                    continue;
                },
                "\"" => Lexer::lex_string_literals(lexeme, &mut peek, line_number),                
                _ => Err(anyhow!(Lexer::lexical_error(lexeme, line_number))),
            }?;
            tokens.push(out);
        }
        Ok(tokens)
    }

    /// keep consuming the set of characters inside of peek until another " character is found or the end of the string is reached which results in an error.
    fn lex_string_literals(lexeme : String, peek : &mut Peekable<Chars>, line_number: u32) -> Result<Token> {
        unimplemented!()
    }

    fn lex_number_literals() -> Result<Token> {
        unimplemented!()
    }  

    

    fn lexical_error(message: String, line_number: u32) -> String {
        format!("{:#?} (line {})", message, line_number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_error_test() {
        todo!()
    }

    #[test]
    fn lexer_lex_line_proper_test() {
        let source_code = "({ )}\n+ - !";
        let mut lexer = Lexer::new();
        todo!()
    }

    #[test]
    fn lex_no_whitespace_test() {
        let source_code = "a+b";
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(source_code).unwrap();

        let expected = vec![
            Token::new(TokenType::Identifier, "a".to_string(), 1),
            Token::new(TokenType::Plus, "+".to_string(), 1),
            Token::new(TokenType::Identifier, "b".to_string(), 1)];

        tokens.iter().zip(expected.iter()).for_each(|(token, expected_token)| {
            assert_eq!(token, expected_token);
        });

        let source_code = "a==b"; 
        let tokens = lexer.lex(source_code).unwrap();

        let source_code = "a!=b";
        let tokens = lexer.lex(source_code).unwrap();

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
