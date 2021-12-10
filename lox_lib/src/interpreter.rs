use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use anyhow::{anyhow, Context, Result};

/// the interpreter is responsible for running lox programs either form a file or a REPL
pub struct Interpreter;

pub enum InterpreterMode {
    Script(String),
    Repl,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run(&self, mode: InterpreterMode) -> Result<()> {
        match mode {
            InterpreterMode::Script(path) => self.run_script(path),
            InterpreterMode::Repl => self.run_repl(),
        }
    }

    pub fn run_script(&self, path: String) -> Result<()> {
        println!("Running script: {}", path);
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read in file from {}", path))?;
        self.run_on_string(source)
    }

    fn run_on_string(&self, source: String) -> Result<()> {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(&source)?;

        let mut parser = Parser::new();
        let node = parser.parse(tokens);
        Ok(())
    }

    pub fn run_repl(&self) -> Result<()> {
        loop {
            let mut buf = String::new();
            print!(">> ");
            let input = std::io::stdin().read_line(&mut buf)?;

            if buf == "" {
                break;
            }
            println!("{:?}", buf);
        }
        Ok(())
    }

    pub fn error(line: u32, message: String) -> String {
        Interpreter::report(line, "".into(), message)
    }

    pub fn report(line: u32, err_where: String, message: String) -> String {
        format!("[line {}] Error {}: {}", line, err_where, message)
    }
}

#[cfg(test)]
mod test {}
