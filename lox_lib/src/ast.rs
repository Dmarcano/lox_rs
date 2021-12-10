//!
//! This module contains the AST for the Lox language.
//!

use crate::lexer::{Token, TokenType};

/// The operators supported by the Lox language.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Add {line : u32},
    Subtract {line : u32},
    Multiply {line : u32},
    Divide {line : u32},
    GreaterThan {line : u32},
    LessThan {line : u32},
    Equal {line : u32},
    EqualEqual {line : u32},
    NotEqual {line : u32},
    And{line : u32},
    Or {line : u32},
    Bang {line : u32},
}

impl TryFrom<&Token> for Operator {
    type Error = String;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let line = token.line; 
        match token.token_type {
            TokenType::Plus => Ok(Operator::Add{line}),
            TokenType::Minus => Ok(Operator::Subtract{line}),
            TokenType::Star => Ok(Operator::Multiply{line}),
            TokenType::Slash => Ok(Operator::Divide{line}),
            TokenType::Greater => Ok(Operator::GreaterThan{line}),
            TokenType::Less => Ok(Operator::LessThan{line}),
            TokenType::Equal => Ok(Operator::Equal{line}),
            TokenType::BangEqual => Ok(Operator::NotEqual{line}),
            TokenType::And => Ok(Operator::And{line}),
            TokenType::Or => Ok(Operator::Or{line}),
            TokenType::Bang => Ok(Operator::Bang{line}),
            TokenType::EqualEqual => Ok(Operator::EqualEqual{line}),
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

impl Literal {
    pub fn is_falsy(&self) -> bool {
        match &self {
            Literal::Number(_) => true,
            Literal::String(_) => true,
            Literal::Boolean(val) => *val,
            Literal::Nil => false,
        }
    }
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
