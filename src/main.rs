use anyhow::{Result, anyhow, Context};
/// the interpreter can be run in one of two modes.
/// either it can be running a single script that is specified or
/// it can be running in interactive mode where it functions as a REPL.
enum InterpreterMode {
    Script(String),
    Repl,
}

/// the interpreter is responsible for running lox programs either form a file or a REPL
struct Interpreter;

impl Interpreter { 
    fn new() -> Self {
        Interpreter
    }

    fn run(&self, mode: InterpreterMode) -> Result<()> {
        match mode {
            InterpreterMode::Script(path) => {self.run_script(path) }
            InterpreterMode::Repl => { self.run_repl()}
        }
    }

    fn run_script(&self, path: String) -> Result<()> {
        let source = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read in file from {}", path))?;

        let mut lexer = Lexer::new();
        let tokens = lexer.lex(&source)?;

        Ok(())
    }

    fn run_repl(&self) -> Result<()> {
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


    fn error(line : u32, message : String, ) -> anyhow::Error {
        Interpreter::report(line, "".into(), message)
    }

    fn report(line : u32, err_where : String, message : String) -> anyhow::Error {
        anyhow!("[line {}] Error {}: {}", line, err_where, message)
    }
}

struct Token; 

/// A lexer (or scanner) is responsible for breaking a program into a sequence of tokens.
struct Lexer; 

impl Lexer { 
    fn new() -> Self {
        Lexer
    }

    // break a stream of bytes into a sequence of tokens.
    fn lex(&self, input: &str) -> Result<Vec<Token>> {
        todo!()
    }
    
}

fn main() -> Result<()> {
    let matches = std::env::args().collect::<Vec<String>>();

    let mode = match matches.len(){
        1 => Ok(InterpreterMode::Repl),
        2 => Ok(InterpreterMode::Script(matches.into_iter().next().unwrap())),
        _ => Err(anyhow!("too many arguments: Usage lox [script]")),
    }?;

    let interpreter = Interpreter::new();
    interpreter.run(mode)?; 
    Ok(())
}


#[cfg(test)]
mod test  { 

}