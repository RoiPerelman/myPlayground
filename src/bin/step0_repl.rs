use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn READ(input: String) -> String {
    return input;
}

fn EVAL(input: String) -> String {
    return input;
}

fn PRINT(input: String) -> String {
    return input;
}

fn rep(input: String) -> String {
    return PRINT(EVAL(READ(input)));
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let line = line.trim();
                println!("{}", rep(line.to_string()));
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
