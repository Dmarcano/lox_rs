//!
//! This module contains the AST for the Lox language.
//!

///
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

///
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

/// A node in the AST.
#[derive(Debug, Clone)]
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
