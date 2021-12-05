use crate::ast::{Literal, Node, Operator};
use crate::lexer::{Token, TokenType};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {}

/*
 Reference Lox Grammar (So far)


    expression     -> equality ;

    equality       -> comparison ( ("!=" | "==") comparison )* ;

    comparison     -> term ( (">" | "<" | "<=", ">=") term )* ;

    term.          -> factor ( ("+" | "-") factor )* ;

    factor         -> unary ( ("*" | "/") unary)* ;

    unary 		    -> ("!" | "-")  unary | primary ;

    primary         -> NUMBER | STRING | "True" | "False" | "Nil" | "("expression")" ;
*/
impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn expression(&self) -> Node {
        self.equality()
    }

    // equality       -> comparison ( ("!=" | "==") comparison )* ;
    pub fn equality(&self) -> Node {
        let mut node = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {}
        todo!()
    }

    pub fn comparison(&self) -> Node {
        todo!()
    }

    pub fn term(&self) -> Node {
        todo!()
    }

    pub fn factor(&self) -> Node {
        todo!()
    }

    pub fn unary(&self) -> Node {
        todo!()
    }

    pub fn primary(&self) -> Node {
        todo!()
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Node {
        todo!()
    }

    /// Tries to match the given tokens to the next token in the Iterator/Stream,
    /// If it matches, then it returns true
    pub fn match_tokens(&self, match_tokens: &[TokenType]) -> bool {
        todo!()
    }
}
