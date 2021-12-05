use crate::ast::{Literal, Node, Operator};
use crate::lexer::{Token, TokenType};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {}

type ParserBinaryFn = fn(&Parser ) -> Node; 

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

    // equality  -> comparison ( ("!=" | "==") comparison )* ;
    pub fn equality(&self) -> Node {
        let mut node = self.comparison();

        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.expression();
            node = Node::BinaryExpr {
                operator: operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        node
    }

    fn binary_expression_match(&self, paring_fn : ParserBinaryFn, token_types : &[TokenType]) -> Node { 
        let mut node = paring_fn(self);

        while let Some(operator) = self.match_tokens(token_types) {
            let right = self.expression();
            node = Node::BinaryExpr {
                operator: operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        node
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
    pub fn match_tokens(&self, match_tokens: &[TokenType]) -> Option<Operator> {
        todo!()
    }
}


#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn expression_test() { 
        todo!()
    }

    #[test]
    fn equality_test() { 
        // testing the equality of the following expression 
        // a == b == c != d
        let tokens = [
            Token::new(TokenType::Identifier, "a".to_string(), 0),
            Token::new(TokenType::EqualEqual, "==".to_string(), 0),
            Token::new(TokenType::Identifier, "b".to_string(), 0),
            Token::new(TokenType::EqualEqual, "==".to_string(), 0),
            Token::new(TokenType::Identifier, "c".to_string(), 0),
            Token::new(TokenType::BangEqual, "!=".to_string(), 0),
            Token::new(TokenType::Identifier, "d".to_string(), 0),
        ];
        todo!()
    }
}
