use std::str::Chars;

use my_mal::reader::read_str;
use my_mal::types::Mal;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn READ(input: String) -> Result<Mal, &'static str> {
    let ast = read_str(&input);
    println!("AST: {:?}", ast);
    ast
}

fn EVAL(input: Result<Mal, &'static str>) -> Result<Mal, &'static str> {
    input
}

fn PRINT(input: Result<Mal, &'static str>) {
    match input {
        Ok(mal) => println!("{}", mal),
        Err(e) => println!("{}", e),
    }
}

fn rep(input: String) {
    PRINT(EVAL(READ(input)));
}

fn main() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let line = line.trim();
                rep(line.to_string());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
    Ok(())
}
