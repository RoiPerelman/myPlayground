use std::io::{self, Write};

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

fn
 main() {
    loop {
        let mut input = String::new();
        print!("user> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            println!("Exiting REPL.");
            break;
        }


        println!("{}", rep(input.to_string()));
    }
}
