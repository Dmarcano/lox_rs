use anyhow::{anyhow, Context, Result};

use lox_lib::{interpreter::{Interpreter, InterpreterMode},lexer::{Lexer, Token}};


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
