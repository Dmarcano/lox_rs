use anyhow::{anyhow, Context, Result};
pub mod interpreter;
/// the interpreter can be run in one of two modes.
/// either it can be running a single script that is specified or
/// it can be running in interactive mode where it functions as a REPL.
pub mod lexer;

use interpreter::{Interpreter, InterpreterMode};

fn main() -> Result<()> {
    let matches = std::env::args().collect::<Vec<String>>();

    let mode = match matches.len() {
        1 => Ok(InterpreterMode::Repl),
        2 => Ok(InterpreterMode::Script(matches.into_iter().nth(1).unwrap())),
        _ => Err(anyhow!("too many arguments: Usage lox [script]")),
    }?;

    let interpreter = Interpreter::new();
    interpreter.run(mode)?;
    Ok(())
}

#[cfg(test)]
mod test {}
