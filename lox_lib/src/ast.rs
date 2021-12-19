//!
//! This module contains the AST for the Lox language.
//!

use crate::lexer::{Token, TokenType};

/// The operators supported by the Lox language.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Add { line: u32 },
    Subtract { line: u32 },
    Multiply { line: u32 },
    Divide { line: u32 },
    GreaterThan { line: u32 },
    LessThan { line: u32 },
    Equal { line: u32 },
    EqualEqual { line: u32 },
    NotEqual { line: u32 },
    And { line: u32 },
    Or { line: u32 },
    Bang { line: u32 },
}

impl TryFrom<&Token> for Operator {
    type Error = String;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let line = token.line;
        match token.token_type {
            TokenType::Plus => Ok(Operator::Add { line }),
            TokenType::Minus => Ok(Operator::Subtract { line }),
            TokenType::Star => Ok(Operator::Multiply { line }),
            TokenType::Slash => Ok(Operator::Divide { line }),
            TokenType::Greater => Ok(Operator::GreaterThan { line }),
            TokenType::Less => Ok(Operator::LessThan { line }),
            TokenType::Equal => Ok(Operator::Equal { line }),
            TokenType::BangEqual => Ok(Operator::NotEqual { line }),
            TokenType::And => Ok(Operator::And { line }),
            TokenType::Or => Ok(Operator::Or { line }),
            TokenType::Bang => Ok(Operator::Bang { line }),
            TokenType::EqualEqual => Ok(Operator::EqualEqual { line }),
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
    pub fn is_equal(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => *a == *b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Boolean(a), Literal::Boolean(b)) => a == b,
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Nil, _) => false,
            (_, Literal::Nil) => false,
            _ => false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct BinaryExpr {
    left: Box<Node>,
    right: Box<Node>,
    operator: Operator,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct UnaryExpr {
    right: Box<Node>,
    operator: Operator,
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

/// The visitor is a trait for parsing and evaluating expressions in an Lox AST made up
/// of recursive nodes
pub trait Visitor {
    type Output;

    /// Visits nodes in the AST by calling the appropriate method for the node type.
    /// Generally it is up to implementors of this trait to implement the specific visiting methods
    /// but only the visit node method should be used to visit nodes themselves
    fn visit_node(&mut self, node: &Node) -> Self::Output {
        match node {
            Node::Literal(literal) => self.visit_literal(literal),
            Node::Grouping(grouping) => self.visit_grouping(grouping),
            Node::UnaryExpr { operator, right } => self.visit_unary_expr(operator, right),
            Node::BinaryExpr {
                left,
                operator,
                right,
            } => self.visit_binary_expr(left, operator, right),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;

    fn visit_grouping(&mut self, grouping: &Node) -> Self::Output;

    fn visit_binary_expr(&mut self, left: &Node, operator: &Operator, right: &Node)
        -> Self::Output;

    fn visit_unary_expr(&mut self, operator: &Operator, child: &Node) -> Self::Output;
}
