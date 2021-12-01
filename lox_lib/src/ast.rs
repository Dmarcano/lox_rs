//!
//! This module contains the AST for the Lox language.
//!
use crate::lexer::Token;


#[derive(Debug, Clone)]
pub enum Expression {
    Primary(Primary),
}

pub enum Term { 
    Factor(Factor),
    Add(Box<Term>, Box<Term>),
    Sub(Box<Term>, Box<Term>),
}


#[derive(Debug)]
/// A factor expression is an expression that uses the `*` or `/` operators. It follows the grammar 
/// 
///  factor => unary (("*" | "/") unary)*
pub enum Factor {
    Unary(Unary),
    Times(Box<Factor>, Box<Factor>),
    Divide(Box<Factor>, Box<Factor>),
}

#[derive(Debug, Clone)]
/// A primary expression contains the literals and the grouping expressions
pub enum Primary {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
    Expression(Box<Expression>),
}

#[derive(Debug)]
/// A unary expression is an expression which follows the grammar
///
/// unary => ("!" | "-") unary | primary
///
pub enum Unary {
    Bang(Box<Unary>),
    Minus(Box<Unary>),
    Primary(Primary),
}
