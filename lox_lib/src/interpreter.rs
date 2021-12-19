use crate::ast::{Literal, ExprNode, Operator, Visitor};
use crate::lexer::{Lexer};
use crate::parser::Parser;
use anyhow::{anyhow, Context, Result};

/// the interpreter is responsible for running lox programs either form a file or a REPL
pub struct Interpreter;

struct RuntimeErr {
    message: String,
    line: u32,
}

pub enum InterpreterMode {
    Script(String),
    Repl,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run(&mut self, mode: InterpreterMode) -> Result<()> {
        match mode {
            InterpreterMode::Script(path) => self.run_script(path),
            InterpreterMode::Repl => self.run_repl(),
        }
    }

    pub fn run_script(&mut self, path: String) -> Result<()> {
        println!("Running script: {}", path);
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read in file from {}", path))?;
        self.run_on_string(source)
    }

    fn run_on_string(&mut self, source: String) -> Result<()> {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(&source)?;

        let mut parser = Parser::new();
        let node = parser.parse(tokens);
        let literal=  self.visit_node(&node)?;
        println!("{:?}", literal);
        Ok(())
    }

    pub fn run_repl(&mut self) -> Result<()> {
        // print!("\n>> ");
        loop {
            let mut buf = String::new();
            
            print!(">> ");
            let _ = std::io::stdin().read_line(&mut buf)?;
            let source = buf.trim().to_string();
            
            println!("{}", source);
            if buf == "" {
                break;
            }
            self.run_on_string(buf.clone())?;
        }
        Ok(())
    }

    pub fn error(line: u32, message: String) -> String {
        Interpreter::report(line, "".into(), message)
    }

    pub fn report(line: u32, err_where: String, message: String) -> String {
        format!("[line {}] Error {}: {}", line, err_where, message)
    }

    fn check_type() -> Result<()> {
        Ok(())
    }

    /// evaluates the addition of a left and right literal and returns the result
    /// for two numbers this is a simple addition
    /// for two strings this is a concatenation of right on the end of left
    fn add_impl(left: Literal, right: Literal, line: u32) -> Result<Literal> {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
            (Literal::String(l), Literal::String(r)) => Ok(Literal::String(l + &r)),
            (Literal::Number(left), _) => {
                return Err(anyhow!(Interpreter::error(
                    line,
                    format!(
                        "the left side number {} operand is being added to non left number",
                        left
                    )
                )))
            }
            (Literal::String(left), _) => {
                return Err(anyhow!(Interpreter::error(
                    line,
                    format!(
                        "the left side string {} operand is being added to non left number",
                        left
                    )
                )))
            }
            _ => {
                return Err(anyhow!(Interpreter::error(
                    line,
                    "Operands must be two numbers or two strings".into()
                )))
            }
        }
    }
}

impl Visitor for Interpreter {
    type Output = Result<Literal>;

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output {
        Ok(literal.clone())
    }

    fn visit_grouping(&mut self, grouping: &ExprNode) -> Self::Output {
        self.visit_node(grouping)
    }

    fn visit_binary_expr(
        &mut self,
        left: &ExprNode,
        operator: &Operator,
        right: &ExprNode,
    ) -> Self::Output {
        let left_literal = self.visit_node(left)?;
        let right_literal = self.visit_node(right)?;

        match operator {
            Operator::Add { line } => Interpreter::add_impl(left_literal, right_literal, *line),
            Operator::Subtract { line } => match (left_literal, right_literal) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                _ => {
                    return Err(anyhow!(Interpreter::error(
                        *line,
                        "Operands must be two numbers".into()
                    )))
                }
            },
            Operator::Multiply { line } => match (left_literal, right_literal) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                _ => {
                    return Err(anyhow!(Interpreter::error(
                        *line,
                        "Operands must be two numbers".into()
                    )))
                }
            },
            Operator::Divide { line } => match (left_literal, right_literal) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
                _ => {
                    return Err(anyhow!(Interpreter::error(
                        *line,
                        "Operands must be two numbers".into()
                    )))
                }
            },
            Operator::GreaterThan { line } => match (left_literal, right_literal) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l > r)),
                (Literal::String(l), Literal::String(r)) => Ok(Literal::Boolean(l > r)),
                _ => {
                    return Err(anyhow!(Interpreter::error(
                        *line,
                        "Operands must be two numbers or two strings".into()
                    )))
                }
            },
            Operator::LessThan { line } => match (left_literal, right_literal) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l < r)),
                (Literal::String(l), Literal::String(r)) => Ok(Literal::Boolean(l < r)),
                _ => {
                    return Err(anyhow!(Interpreter::error(
                        *line,
                        "Operands must be two numbers or two strings".into()
                    )))
                }
            },
            Operator::Equal { line: _ } => todo!("only expressions are supported!"),
            Operator::EqualEqual { line: _ } => {
                Ok(Literal::Boolean(left_literal.is_equal(&right_literal)))
            }
            Operator::NotEqual { line: _ } => {
                Ok(Literal::Boolean(!left_literal.is_equal(&right_literal)))
            }
            Operator::And { line: _ } => todo!("only expressions are supported!"),
            Operator::Or { line: _ } => todo!("only expressions are supported!"),
            _ => return Err(anyhow!("Unsupported operator")),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Operator, child: &ExprNode) -> Self::Output {
        let output = self.visit_node(child)?;

        match operator {
            Operator::Bang { line: _ } => return Ok(Literal::Boolean(!output.is_falsy())),
            Operator::Subtract { line } => {
                if let Literal::Number(value) = output {
                    return Ok(Literal::Number(-value));
                } else {
                    return Err(anyhow!(format!(
                        "Unary operator '-' can only be applied to numbers on line {}",
                        line
                    )));
                }
            }
            _ => Err(anyhow!(format!(
                "Unexpected operator of type {:?} in an Unary expression. Only",
                operator
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // tests that the lexer and the parser and interpreter can all be used together to generate
    // the expected output from some lox source code.
    fn interpreter_integration_test_expression() {}

    #[test]
    fn unary_expr_test() {
        let expr = "!5";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(false));

        let expr = "-2"; 
        let result = get_parsed_expr(expr); 
        assert_eq!(result, Literal::Number(-2.0));
    }

    #[test]
    fn add_sub_expr_test() { 
        let expr = "1 + 2"; 
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(3.0));

        let expr = "1 - 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(-1.0));

        let expr = "1 + 2 - 3";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(0.0));
    }

    #[test]
    fn mul_sub_test() { 
        let expr = "1 * 2"; 
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(2.0));

        let expr = "1 / 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(0.5));

        let expr = "1 * 2 / 3";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(2.0/3.0));
    }

    #[test]
    fn grouping_test() { 
        let expr = "1 +(2 * 3)";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(7.0));

        let expr = "(1 + 2) * 3";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(9.0));

        let expr = "(1 + 2) * (3 + 4)";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(21.0));

        let expr = "3 * (1 + 2)";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Number(9.0));
    }

    #[test]
    /// tests that the ">", "<", ">=", and "<=" operators work as expected.
    fn greater_less_than_tests() { 
        let expr = "1 > 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(false));

        let expr = "1 < 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(true));

        let expr = "1 >= 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(false));

        let expr = "1 <= 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(true));
    }

    #[test]
    /// tests that the "==" and "!=" operators work as expected.
    fn equals_equals_test() { 
        let expr = "1 == 1";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(true));

        let expr = "1 == 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(false));

        let expr = "1 != 2";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(true));

        let expr = "1 != 1";
        let result = get_parsed_expr(expr);
        assert_eq!(result, Literal::Boolean(false));
    }


    fn get_parsed_expr(expr: &str) -> Literal {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(expr).unwrap();
        let mut parser = Parser::new();
        let node = parser.parse(tokens);
        let mut interpreter = Interpreter::new();
        interpreter.visit_node(&node).unwrap()
    }
}
