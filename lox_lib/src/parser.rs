use crate::ast::{Literal, Node, Operator};
use crate::lexer::{Token, TokenType};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {
    panic_mode: bool,
    errors: Vec<String>,
}

type ParserBinaryFn = fn(&mut Parser, &mut Vec<Token>) -> Node;

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
        Self {
            panic_mode: false,
            errors: Vec::new(),
        }
    }

    /// This function is used to simplify the implementation of binary expressions. By taking  
    /// advantage of the fact that the grammar for most binary expressions is very similiar
    ///
    fn binary_expression_match(
        &mut self,
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

    fn expression(&mut self, tokens: &mut Vec<Token>) -> Node {
        self.equality(tokens)
    }

    /// Performs a binary equality operation on possible expressions. It follows the following grammar.
    ///
    ///
    /// `equality  -> comparison ( ("!=" | "==") comparison )* ;`
    fn equality(&mut self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(
            Parser::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
            tokens,
        )
    }

    ///  comparison -> term ( (">" | "<" | "<=", ">=") term )* ;
    fn comparison(&mut self, tokens: &mut Vec<Token>) -> Node {
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
    fn term(&mut self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(Parser::factor, &[TokenType::Plus, TokenType::Minus], tokens)
    }

    /// factor -> unary ( ("*" | "/") unary)* ;
    fn factor(&mut self, tokens: &mut Vec<Token>) -> Node {
        self.binary_expression_match(Parser::unary, &[TokenType::Star, TokenType::Slash], tokens)
    }

    fn unary(&mut self, tokens: &mut Vec<Token>) -> Node {
        if let Some(operator) =
            self.match_operator_tokens(&[TokenType::Bang, TokenType::Minus], tokens)
        {
            let right = self.unary(tokens);
            return Node::UnaryExpr {
                operator: operator,
                right: Box::new(right),
            };
        };
        self.primary(tokens)
    }

    // primary -> NUMBER | STRING | "True" | "False" | "Nil" | "("expression")" ;
    fn primary(&mut self, tokens: &mut Vec<Token>) -> Node {
        return self.match_literals(tokens);
    }

    /// Tries to parse a node from the token stream
    pub fn parse(&mut self, mut tokens: Vec<Token>) -> Node {
        return self.expression(&mut tokens);
    }

    // TODO this is whack. Needs more type safety to prevent the wrong token type from being passed in and silently
    // failing.
    //
    /// Tries to match the given tokens to the next token in the Iterator/Stream,
    /// if the tokens match, it returns the operator token, otherwise it returns None/
    ///
    /// ### Panics
    /// If the given tokens are not some sort of operator
    fn match_operator_tokens(
        &self,
        match_tokens: &[TokenType],
        tokens: &mut Vec<Token>,
    ) -> Option<Operator> {
        let mut out = None;

        if let Some(token) = tokens.get(0) {
            if match_tokens.contains(&token.token_type) {
                out = Some(Operator::try_from(token).unwrap());
            }
        }
        // this second match is done to remove the matched token from the iterator
        match out {
            Some(operator) => {
                // TODO: using a Vec leads to constant O(n) time complexity for every match.
                // quick fix is to use Deque
                tokens.remove(0);
                Some(operator)
            }
            None => None,
        }
    }

    fn match_literals(&mut self, tokens: &mut Vec<Token>) -> Node {
        let mut node: Option<Node> = None;

        if let Some(token) = tokens.get(0) {
            match &token.token_type {
                TokenType::Number(number) => node = Some(Node::Literal(Literal::Number(*number))),
                TokenType::String(string) => {
                    node = Some(Node::Literal(Literal::String(string.clone())))
                }
                TokenType::False => node = Some(Node::Literal(Literal::Boolean(false))),
                TokenType::True => node = Some(Node::Literal(Literal::Boolean(true))),
                TokenType::Nil => node = Some(Node::Literal(Literal::Nil)),
                _ => {
                    // do nothing in case of left parenthesis which needs a mutable reference to tokens
                }
            }
        }

        if let Some(literal_node) = node {
            tokens.remove(0);
            return literal_node;
        }

        if tokens[0].token_type == TokenType::LeftParen {
            tokens.remove(0);
            let expr = self.expression(tokens);
            if tokens[0].token_type == TokenType::RightParen {
                tokens.remove(0);
                return Node::Grouping(Box::new(expr));
            } else {
                self.panic_mode = true;
                self.send_err("Expected ')' after expression");
                // TODO Synchronize
            }
        }

        todo!("an unsupported token was found! {:?}", tokens.get(0))
    }

    fn send_err<T: ToString>(&mut self, message: T) {
        self.errors.push(message.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grouping_test() {
        todo!()
    }

    #[test]
    fn unary_binary_expression_test() {
        // testing the node created from the following expression
        // 1 + 2 * -3

        // (1) + (2 * (-3) )
        let mut parser = Parser::new();
        let tokens = vec![
            Token::new(TokenType::Number(1.0), 1.to_string(), 1),
            Token::new(TokenType::Plus, 1.to_string(), 1 ),
            Token::new(TokenType::Number(2.0), 1.to_string(), 1),
            Token::new(TokenType::Star, 1.to_string(), 1),
            Token::new(TokenType::Minus, 1.to_string(), 1),
            Token::new(TokenType::Number(3.0), 1.to_string(), 1),
        ];
        let node = parser.parse(tokens);
        let expected_node = Node::BinaryExpr {
            operator: Operator::Add{line : 1},
            left: Box::new(Node::Literal(Literal::Number(1.0))),
            right: Box::new(Node::BinaryExpr {
                operator: Operator::Multiply{line : 1},
                left: Box::new(Node::Literal(Literal::Number(2.0))),
                right: Box::new(Node::UnaryExpr {
                    operator: Operator::Subtract{line : 1},
                    right: Box::new(Node::Literal(Literal::Number(3.0))),
                }),
            }),
        };

        assert_eq!(node, expected_node);
    }

    #[test]
    fn crafting_interpreters_example_test() {
        // testing the node created from the following expression
        // 6 / 3 - 1
        let mut parser = Parser::new();
        let tokens = vec![
            Token::new(TokenType::Number(6.0), 1.to_string(), 1),
            Token::new(TokenType::Slash, 1.to_string(), 1),
            Token::new(TokenType::Number(3.0), 1.to_string(), 1),
            Token::new(TokenType::Minus, 1.to_string(), 1),
            Token::new(TokenType::Number(1.0), 1.to_string(), 1),
        ];
        let node = parser.parse(tokens);

        let expected_node = Node::BinaryExpr {
            left: Box::new(Node::BinaryExpr {
                left: Box::new(Node::Literal(Literal::Number(6.0))),
                operator: Operator::Divide{line : 1},
                right: Box::new(Node::Literal(Literal::Number(3.0))),
            }),

            operator: Operator::Subtract{line : 1},
            right: Box::new(Node::Literal(Literal::Number(1.0))),
        };

        assert_eq!(expected_node, node);
    }

    #[test]
    fn equality_test() {
        // testing the equality of the following expression
        // 'a' == 'b'
        let tokens = [
            Token::new(TokenType::String("a".to_string()), "a".to_string(), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), 1),
            Token::new(TokenType::String("b".to_string()), "b".to_string(), 1),
        ]
        .to_vec();
        let expected_node = Node::BinaryExpr {
            operator: Operator::EqualEqual{line : 1},
            left: Box::new(Node::Literal(Literal::String("a".to_string()))),
            right: Box::new(Node::Literal(Literal::String("b".to_string()))),
        };
        let mut parser = Parser::new();
        let node = parser.parse(tokens);
        assert_eq!(node, expected_node);

        // testing the equality of the following expression
        // 1 != 2 == 3 != 'b'
        let _tokens = [
            Token::new(TokenType::Number(1.0), "1".to_string(), 0),
            Token::new(TokenType::BangEqual, "!=".to_string(), 0),
            Token::new(TokenType::Number(2.0), "2".to_string(), 0),
            Token::new(TokenType::EqualEqual, "==".to_string(), 0),
            Token::new(TokenType::Number(3.0), "3".to_string(), 0),
            Token::new(TokenType::BangEqual, "!=".to_string(), 0),
            Token::new(TokenType::String("b".to_string()), "b".to_string(), 0),
        ]
        .to_vec();
    }
}
