//!
//! This module contains the AST for the Lox language.
//!
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Operator { 
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    And,
    Or,
    Bang, 
}


#[derive(Debug, Clone)]
pub enum Node { 
    String(String),
    Number(f32),
    True, 
    False, 
    Nil, 
    Grouping(Box<Node>),
    UnaryExpr{ 
        operator: Operator, 
        right: Box<Node> 
    },
    BinaryExpr{ 
        left: Box<Node>, 
        operator: Operator, 
        right: Box<Node> 
    },
}