use crate::ast::{ExprNode, Literal, Operator, StmtNode};
use crate::lexer::{Token, TokenType};
use anyhow::{anyhow, Result};

/// a parser for the Lox language. It creates an Abstract Syntax Tree (AST) from a token stream.
pub struct Parser {
    panic_mode: bool,
    errors: Vec<String>,
}

type ParserBinaryFn = fn(&mut Parser, &mut Vec<Token>) -> Result<ExprNode>;

/*
 Reference Lox Expression Grammar (So far)


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
    /// Keeps matching the tokens in the given tokens vector, to any of the token_types passed in
    /// removing them from the vector as they are matched.
    ///
    fn binary_expression_match(
        &mut self,
        precedence_fn: ParserBinaryFn,
        token_types: &[TokenType],
        tokens: &mut Vec<Token>,
    ) -> Result<ExprNode> {
        let mut node = precedence_fn(self, tokens)?;

        while let Some(operator) = self.match_operator_tokens(token_types, tokens) {
            let right = precedence_fn(self, tokens)?;
            node = ExprNode::BinaryExpr {
                operator: operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    pub(crate) fn expression(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        self.equality(tokens)
    }

    /// Performs a binary equality operation on possible expressions. It follows the following grammar.
    ///
    ///
    /// `equality  -> comparison ( ("!=" | "==") comparison )* ;`
    fn equality(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        self.binary_expression_match(
            Parser::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
            tokens,
        )
    }

    ///  comparison -> term ( (">" | "<" | "<=", ">=") term )* ;
    fn comparison(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
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
    fn term(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        self.binary_expression_match(Parser::factor, &[TokenType::Plus, TokenType::Minus], tokens)
    }

    /// factor -> unary ( ("*" | "/") unary)* ;
    fn factor(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        self.binary_expression_match(Parser::unary, &[TokenType::Star, TokenType::Slash], tokens)
    }

    fn unary(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        if let Some(operator) =
            self.match_operator_tokens(&[TokenType::Bang, TokenType::Minus], tokens)
        {
            let right = self.unary(tokens)?;
            let expr = ExprNode::UnaryExpr {
                operator: operator,
                right: Box::new(right),
            };
            return Ok(expr);
        };
        self.primary(tokens)
    }

    // primary -> NUMBER | STRING | "True" | "False" | "Nil" | "("expression")" ;
    fn primary(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        return self.match_literals(tokens);
    }

    fn print_stmt(&mut self, tokens: &mut Vec<Token>) -> Result<StmtNode> {
        let _ = tokens.remove(0); // remove print token
        let expr = self.expression(tokens)?;
        match Parser::consume(TokenType::Semicolon, tokens) {
            Ok(_) => Ok(StmtNode::PrintStmt(expr)),
            Err(_) => Err(anyhow!("Expected ';' after a statement")),
        }
    }

    /// This function will match tokens with the possible
    pub(crate) fn statement(&mut self, tokens: &mut Vec<Token>) -> StmtNode {
        // again using a Dequeue would make this much faster
        if Parser::match_token(
            TokenType::Print,
            tokens.get(0).expect("No tokens in statement"),
        ) {
            match self.print_stmt(tokens) {
                Ok(print_stmt) => return print_stmt,
                Err(err) => StmtNode::ErrStmt(err.to_string()),
            }
        } else {
            match self.expression(tokens) {
                Ok(expr) => match Parser::consume(TokenType::Semicolon, tokens) {
                    Ok(_) => StmtNode::ExprStmt(expr),
                    Err(_) => {
                        StmtNode::ErrStmt(anyhow!("Expected ';' after an expression").to_string())
                    }
                },
                Err(err) => StmtNode::ErrStmt(err.to_string()),
            }
        }
    }

    /// returns true or false if the token matches the token_type that is passed in
    fn match_token(token_type: TokenType, tokens: &Token) -> bool {
        return tokens.token_type == token_type;
    }

    /// consumes a token from the tokens vector stream if it matches the TokenType that is expected passed in
    /// otherwise returns an error with the actual
    fn consume(expected_token: TokenType, tokens: &mut Vec<Token>) -> Result<()> {
        // token is not copy because of the the string literal not being copy. Otherwise clones are fine and
        // not expected to do much here
        let token_match = Parser::match_token(
            expected_token.clone(),
            tokens.get(0).expect("Expected token in fn consume"),
        );

        if token_match {
            // TODO use a dequeue here for easy speedups
            tokens.remove(0);
            return Ok(());
        } else {
            return Err(anyhow!(
                "Expected token {:?} but got {:?} token",
                expected_token,
                tokens.get(0).unwrap().token_type
            ));
        }
    }

    /// Generates a syntax tree from a stream of tokens.
    pub fn parse(&mut self, mut tokens: Vec<Token>) -> Vec<StmtNode> {
        let mut statements = Vec::new();

        while tokens.len() > 0 {
            let statement = self.statement(&mut tokens);
            statements.push(statement);
        }
        statements
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

    fn match_literals(&mut self, tokens: &mut Vec<Token>) -> Result<ExprNode> {
        let mut node: Option<ExprNode> = None;

        if let Some(token) = tokens.get(0) {
            match &token.token_type {
                TokenType::Number(number) => {
                    node = Some(ExprNode::Literal(Literal::Number(*number)))
                }
                TokenType::String(string) => {
                    node = Some(ExprNode::Literal(Literal::String(string.clone())))
                }
                TokenType::False => node = Some(ExprNode::Literal(Literal::Boolean(false))),
                TokenType::True => node = Some(ExprNode::Literal(Literal::Boolean(true))),
                TokenType::Nil => node = Some(ExprNode::Literal(Literal::Nil)),
                _ => {
                    // do nothing in case of left parenthesis which needs a mutable reference to tokens
                }
            }
        }

        if let Some(literal_node) = node {
            tokens.remove(0);
            return Ok(literal_node);
        }

        if tokens[0].token_type == TokenType::LeftParen {
            tokens.remove(0);
            let expr = self.expression(tokens)?;
            if tokens[0].token_type == TokenType::RightParen {
                tokens.remove(0);
                return Ok(ExprNode::Grouping(Box::new(expr)));
            } else {
                self.panic_mode = true;
                self.send_err("Expected ')' after expression");
                // TODO Synchronize
            }
        }

        self.panic_mode = true;
        self.send_err(format!(
            "an unsupported token was found! {:?}",
            tokens.get(0)
        ));

        Err(anyhow!(format!(
            "unsupported token {:?} in expression",
            tokens.get(0).unwrap().token_type
        )))
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
    /// testing that a statement without a semicolon returns errors as expected
    fn statement_semi_colon_test() {}

    #[test]
    /// tests that using a print statement and an expression statement returns the expected syntax tree
    fn statement_test() {
        // "print(\"hello world\")";
        let mut tokens = vec![
            Token::new(TokenType::Print, "print".to_string(), 1),
            Token::new(TokenType::LeftParen, "(".to_string(), 1),
            Token::new(
                TokenType::String("\"hello world\"".to_string()),
                "\"hello world\"".to_string(),
                1,
            ),
            Token::new(TokenType::RightParen, ")".to_string(), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), 1),
            Token::new(TokenType::Eof, "".to_string(), 1),
        ];

        let mut parser = Parser::new();
        let node = parser.statement(&mut tokens);
        let expected_node = StmtNode::PrintStmt(ExprNode::Grouping(Box::new(ExprNode::Literal(
            Literal::String("\"hello world\"".to_string()),
        ))));
        assert_eq!(node, expected_node);
    }

    #[test]
    fn unary_binary_expression_test() {
        // testing the node created from the following expression
        // 1 + 2 * -3

        // (1) + (2 * (-3) )
        let mut parser = Parser::new();
        let mut tokens = vec![
            Token::new(TokenType::Number(1.0), 1.to_string(), 1),
            Token::new(TokenType::Plus, 1.to_string(), 1),
            Token::new(TokenType::Number(2.0), 1.to_string(), 1),
            Token::new(TokenType::Star, 1.to_string(), 1),
            Token::new(TokenType::Minus, 1.to_string(), 1),
            Token::new(TokenType::Number(3.0), 1.to_string(), 1),
        ];
        let node = parser.expression(&mut tokens).unwrap();
        let expected_node = ExprNode::BinaryExpr {
            operator: Operator::Add { line: 1 },
            left: Box::new(ExprNode::Literal(Literal::Number(1.0))),
            right: Box::new(ExprNode::BinaryExpr {
                operator: Operator::Multiply { line: 1 },
                left: Box::new(ExprNode::Literal(Literal::Number(2.0))),
                right: Box::new(ExprNode::UnaryExpr {
                    operator: Operator::Subtract { line: 1 },
                    right: Box::new(ExprNode::Literal(Literal::Number(3.0))),
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
        let mut tokens = vec![
            Token::new(TokenType::Number(6.0), 1.to_string(), 1),
            Token::new(TokenType::Slash, 1.to_string(), 1),
            Token::new(TokenType::Number(3.0), 1.to_string(), 1),
            Token::new(TokenType::Minus, 1.to_string(), 1),
            Token::new(TokenType::Number(1.0), 1.to_string(), 1),
        ];
        let node = parser.expression(&mut tokens).unwrap();

        let expected_node = ExprNode::BinaryExpr {
            left: Box::new(ExprNode::BinaryExpr {
                left: Box::new(ExprNode::Literal(Literal::Number(6.0))),
                operator: Operator::Divide { line: 1 },
                right: Box::new(ExprNode::Literal(Literal::Number(3.0))),
            }),

            operator: Operator::Subtract { line: 1 },
            right: Box::new(ExprNode::Literal(Literal::Number(1.0))),
        };

        assert_eq!(expected_node, node);
    }

    #[test]
    fn equality_test() {
        // testing the equality of the following expression
        // 'a' == 'b'
        let mut tokens = [
            Token::new(TokenType::String("a".to_string()), "a".to_string(), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), 1),
            Token::new(TokenType::String("b".to_string()), "b".to_string(), 1),
        ]
        .to_vec();
        let expected_node = ExprNode::BinaryExpr {
            operator: Operator::EqualEqual { line: 1 },
            left: Box::new(ExprNode::Literal(Literal::String("a".to_string()))),
            right: Box::new(ExprNode::Literal(Literal::String("b".to_string()))),
        };
        let mut parser = Parser::new();
        let node = parser.expression(&mut tokens).unwrap();
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
