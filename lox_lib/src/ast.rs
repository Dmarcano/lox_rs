//!
//! This module contains the AST for the Lox language.
//!

use crate::lexer::{Token, TokenType};

/// The operators supported by the Lox language.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
    EqualEqual,
    NotEqual,
    And,
    Or,
    Bang,
}

impl TryFrom<&Token> for Operator {
    type Error = String;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token.token_type {
            TokenType::Plus => Ok(Operator::Add),
            TokenType::Minus => Ok(Operator::Subtract),
            TokenType::Star => Ok(Operator::Multiply),
            TokenType::Slash => Ok(Operator::Divide),
            TokenType::Greater => Ok(Operator::GreaterThan),
            TokenType::Less => Ok(Operator::LessThan),
            TokenType::Equal => Ok(Operator::Equal),
            TokenType::BangEqual => Ok(Operator::NotEqual),
            TokenType::And => Ok(Operator::And),
            TokenType::Or => Ok(Operator::Or),
            TokenType::Bang => Ok(Operator::Bang),
            TokenType::EqualEqual => Ok(Operator::EqualEqual),
            _ => Err(format!("{:?} is not an operator", token.token_type)),
        }
    }
}

///
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

/// A node in the AST.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Node {
    Literal(Literal),
    Grouping(Box<Node>),
    UnaryExpr {
        operator: Operator,
        right: Box<Node>,
    },
    BinaryExpr {
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>,
    },
}
