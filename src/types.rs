use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum Mal {
    List(Vec<Mal>),
    Vector(Vec<Mal>),
    HashMap(HashMap<String, Mal>),
    Symbol(String),
    Number(i64),
    String(String),
    // Add more types as needed
}

impl fmt::Display for Mal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mal::List(list) => {
                write!(f, "(")?;
                let mut first = true;
                for item in list {
                    if !first {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, ")")
            }
            Mal::Vector(vec) => {
                write!(f, "[")?;
                let mut first = true;
                for item in vec {
                    if !first {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, "]")
            }
            Mal::HashMap(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in map {
                    if !first {
                        write!(f, " ")?;
                    }
                    if key.starts_with(':') {
                        write!(f, "{} {}", key, value)?;
                    } else {
                        write!(f, "\"{}\" {}", key, value)?;
                    }
                    first = false;
                }
                write!(f, "}}")
            }
            Mal::Symbol(s) => write!(f, "{}", s),
            Mal::Number(n) => write!(f, "{}", n),
            Mal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}
