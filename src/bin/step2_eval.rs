use my_mal::env::Env;
use my_mal::env::initialize_env;
use my_mal::reader::read_str;
use my_mal::types::Mal;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn read(input: String) -> Result<Mal, &'static str> {
    let ast = read_str(&input);
    ast
}

fn eval(input: Result<Mal, &'static str>, env: &Env) -> Result<Mal, &'static str> {
    let ast = input?;

    if let Some(flag) = env.get("DEBUG-EVAL") {
        let should_print = match flag {
            Mal::Nil => false,
            Mal::Symbol(s) if s == "false" => false,
            _ => true,
        };

        if should_print {
            println!("EVAL: {}", ast);
        }
    }

    match &ast {
        Mal::List(l) => {
            if l.is_empty() {
                return Ok(ast.clone());
            }
            let a0 = &l[0];
            let f = eval(Ok(a0.clone()), env)?;
            println!("Function: {:?}", f);
            let mut args: Vec<Mal> = vec![];
            for i in 1..l.len() {
                args.push(eval(Ok(l[i].clone()), env)?);
            }
            match f {
                Mal::Func(func) => Ok(func(args)),
                _ => Err("First element is not a function".into()),
            }
        }
        Mal::Symbol(sym) => env.get(sym).cloned().ok_or("symbol not found"),
        Mal::Vector(v) => {
            // Recursively eval each element of the vector
            let mut new_v = Vec::with_capacity(v.len());
            for elem in v {
                new_v.push(eval(Ok(elem.clone()), env)?);
            }
            Ok(Mal::Vector(new_v))
        }
        Mal::HashMap(hm) => {
            let mut new_hm = std::collections::HashMap::with_capacity(hm.len());
            for (k, v) in hm {
                new_hm.insert(k.clone(), eval(Ok(v.clone()), env)?);
            }
            Ok(Mal::HashMap(new_hm))
        }
        Mal::Number(_) | Mal::String(_) | Mal::Nil => Ok(ast),
        _ => Err("Unsupported AST node".into()),
    }
}

fn print(input: Result<Mal, &'static str>) {
    match input {
        Ok(mal) => println!("{}", mal),
        Err(e) => println!("{}", e),
    }
}

fn rep(input: String, env: &Env) {
    let ast = read(input);
    let evaluated = eval(ast, env);
    print(evaluated);
}

fn main() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut repl_env = initialize_env();

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let line = line.trim();
                rep(line.to_string(), &repl_env);
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
