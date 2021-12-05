use crate::ast::{Literal, Node, Operator};
use crate::lexer::{Token, TokenType};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {}

type ParserBinaryFn<'parser, I: Iterator<Item =&'parser Token>> = fn(& Parser, tokens : &mut I) -> Node;

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

    /// This function is used to simplify the implementation of binary expressions. By taking  
    /// advantage of the fact that the grammar for most binary expressions is very similiar they 
    /// 
    /// 1. Call some function that follows the precedence rules for Lox. This function will return some AST Node
    /// 2. Try to match against 
    /// 
    fn binary_expression_match<'parser, I>(
        &'parser self,
        precedence_fn: ParserBinaryFn<&'parser I>,
        token_types: &[TokenType],
        tokens:  &mut &'parser I,
    ) -> Node 
    where I : Iterator<Item = &'parser Token>
    {
        let mut node = precedence_fn( self, tokens);

        while let Some(operator) = self.match_tokens(token_types) {
            let right = precedence_fn( self, tokens);
            node = Node::BinaryExpr {
                operator: operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        node
    }


    pub fn expression<'parser>(&self, tokens : Vec<Token>) -> Node {
        let iter = tokens.iter();
        self.equality(&mut iter)
    }

    /// Performs a binary equality operation on possible expressions. It follows the following grammar.
    /// 
    /// 
    /// `equality  -> comparison ( ("!=" | "==") comparison )* ;`
    pub fn equality<'parser, I> (&self, tokens:  &mut &'parser I) -> Node 
    where I : Iterator<Item = &'parser Token>
    {

        todo!()
        // self.binary_expression_match(
        //     Parser::comparison::<_> as ParserBinaryFn<_>,
        //     &[TokenType::BangEqual, TokenType::EqualEqual],
        //     tokens,
        // )
    }


    pub fn comparison<'parser, I>(&self, tokens:  &'parser mut I) -> Node 
    where I : Iterator<Item = &'parser Token>
    {
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
        let _iter = match_tokens.iter();
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
