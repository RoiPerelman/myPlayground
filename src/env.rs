use std::collections::HashMap;

use crate::types::Mal;

pub type Env = HashMap<String, Mal>;

pub fn initialize_env() -> Env {
    let mut env = Env::new();
    env.insert("+".to_string(), Mal::Func(|a| int_op(|i, j| i + j, a)));
    env.insert("-".to_string(), Mal::Func(|a| int_op(|i, j| i - j, a)));
    env.insert("*".to_string(), Mal::Func(|a| int_op(|i, j| i * j, a)));
    env.insert("/".to_string(), Mal::Func(|a| int_op(|i, j| i / j, a)));

    // debug flag
    env.insert("DEBUG-EVAL".to_string(), Mal::String("true".to_string()));
    env
}

fn int_op(op: fn(i64, i64) -> i64, a: Vec<Mal>) -> Mal {
    match (a[0].clone(), a[1].clone()) {
        (Mal::Number(a0), Mal::Number(a1)) => Mal::Number(op(a0, a1)),
        _ => Mal::String("Invalid operation".to_string()),
    }
}
