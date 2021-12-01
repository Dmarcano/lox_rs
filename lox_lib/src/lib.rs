#![allow(dead_code)]

pub mod interpreter;

mod ast;
/// the interpreter can be run in one of two modes.
/// either it can be running a single script that is specified or
/// it can be running in interactive mode where it functions as a REPL.
pub mod lexer;
pub mod parser;
