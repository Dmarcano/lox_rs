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
            TokenType::GreaterEqual => Ok(Operator::GreaterThan { line }),
            TokenType::LessEqual => Ok(Operator::LessThan { line }),
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
    left: Box<ExprNode>,
    right: Box<ExprNode>,
    operator: Operator,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct UnaryExpr {
    right: Box<ExprNode>,
    operator: Operator,
}

/// A node in the AST.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprNode {
    Literal(Literal),
    Grouping(Box<ExprNode>),
    UnaryExpr {
        operator: Operator,
        right: Box<ExprNode>,
    },
    BinaryExpr {
        left: Box<ExprNode>,
        operator: Operator,
        right: Box<ExprNode>,
    },
}

/// The visitor is a trait for parsing and evaluating expressions in an Lox AST made up
/// of recursive nodes
pub trait Visitor {
    type Output;

    /// Visits nodes in the AST by calling the appropriate method for the node type.
    /// Generally it is up to implementors of this trait to implement the specific visiting methods
    /// but only the visit node method should be used to visit nodes themselves
    fn visit_node(&mut self, node: &ExprNode) -> Self::Output {
        match node {
            ExprNode::Literal(literal) => self.visit_literal(literal),
            ExprNode::Grouping(grouping) => self.visit_grouping(grouping),
            ExprNode::UnaryExpr { operator, right } => self.visit_unary_expr(operator, right),
            ExprNode::BinaryExpr {
                left,
                operator,
                right,
            } => self.visit_binary_expr(left, operator, right),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;

    fn visit_grouping(&mut self, grouping: &ExprNode) -> Self::Output;

    fn visit_binary_expr(&mut self, left: &ExprNode, operator: &Operator, right: &ExprNode)
                         -> Self::Output;

    fn visit_unary_expr(&mut self, operator: &Operator, child: &ExprNode) -> Self::Output;
}
