use crate::ast::{Literal, Node, Operator};
use crate::lexer::{Token, TokenType};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {}

type ParserBinaryFn = fn(&Parser, &mut Vec<Token>) -> Node;

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
    /// advantage of the fact that the grammar for most binary expressions is very similiar
    ///
    fn binary_expression_match(
        &self,
        precedence_fn: ParserBinaryFn,
        token_types: &[TokenType],
        tokens: &mut Vec<Token>,
    ) -> Node {
        let mut node = precedence_fn(self, tokens);

        while let Some(operator) = self.match_operator_tokens(token_types, tokens) {
            let right = precedence_fn(self, tokens);
            node = Node::BinaryExpr {
                operator: operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        node
    }

    pub fn expression(&self, mut tokens: Vec<Token>) -> Node {
        self.equality(&mut tokens)
    }

    /// Performs a binary equality operation on possible expressions. It follows the following grammar.
    ///
    ///
    /// `equality  -> comparison ( ("!=" | "==") comparison )* ;`
    pub fn equality(&self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(
            Parser::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
            tokens,
        )
    }

    ///  comparison -> term ( (">" | "<" | "<=", ">=") term )* ;
    pub fn comparison(&self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(
            Parser::term,
            &[
                TokenType::Less,
                TokenType::Greater,
                TokenType::LessEqual,
                TokenType::GreaterEqual,
            ],
            tokens,
        )
    }

    /// term -> factor ( ("+" | "-") factor )* ;
    pub fn term(&self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(Parser::factor, &[TokenType::Plus, TokenType::Minus], tokens)
    }

    /// factor -> unary ( ("*" | "/") unary)* ;
    pub fn factor(&self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(Parser::unary, &[TokenType::Star, TokenType::Slash], tokens)
    }

    pub fn unary(&self, tokens: &mut Vec<Token>) -> Node {
        if let Some(operator) = self.match_operator_tokens(&[TokenType::Bang, TokenType::Minus], tokens) {
            let right = self.unary(tokens);
            return Node::UnaryExpr {
                operator: operator,
                right: Box::new(right),
            };
        };
        self.primary(tokens)
    }

    pub fn primary(&self, tokens : &mut Vec<Token>) -> Node {
        todo!()
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Node {
        todo!()
    }

    /// Tries to match the given tokens to the next token in the Iterator/Stream,
    /// If it matches, then it returns true
    pub fn match_operator_tokens(
        &self,
        match_tokens: &[TokenType],
        tokens: &mut Vec<Token>,
    ) -> Option<Operator> {

        let mut out = None; 

        if let Some(token) = tokens.get(0) {
            if match_tokens.contains(&token.token_type) {
                out =  Some(Operator::try_from(token).unwrap());
            }
        }
        // this second match is done to remove the matched token from the iterator
        match out { 
            Some(operator) => {
                // TODO: using a Vec leads to constant O(n) time complexity for every match. 
                // quick fix is to use Deque
                tokens.remove(0);
                Some(operator)
            },
            None => None
        }
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
    fn match_tokens_test() {
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
