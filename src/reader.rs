use std::collections::HashMap;

use crate::{tokenize_regex::tokenize, types::Mal};

pub struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    pub fn peek(&self) -> Option<&String> {
        self.tokens.get(self.pos)
    }
    pub fn next(&mut self) -> Option<&String> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn read_str(input: &str) -> Result<Mal, &'static str> {
    let tokens = tokenize(input);
    let mut reader = Reader { tokens, pos: 0 };
    read_form(&mut reader)
}

pub fn read_form(reader: &mut Reader) -> Result<Mal, &'static str> {
    match reader.peek() {
        Some(token) => match token.as_str() {
            "(" => read_list(reader),
            "[" => read_vector(reader),
            "{" => read_hashmap(reader),
            "'" => {
                reader.next(); // consume the quote
                let form = read_form(reader)?;
                Ok(Mal::List(vec![Mal::Symbol("quote".to_string()), form]))
            }
            "`" => {
                reader.next(); // consume the backquote
                let form = read_form(reader)?;
                Ok(Mal::List(vec![Mal::Symbol("quasiquote".to_string()), form]))
            }
            "~" => {
                reader.next(); // consume the tilde
                let form = read_form(reader)?;
                Ok(Mal::List(vec![Mal::Symbol("unquote".to_string()), form]))
            }
            "~@" => {
                reader.next(); // consume the unquote-splicing
                let form = read_form(reader)?;
                Ok(Mal::List(vec![
                    Mal::Symbol("splice-unquote".to_string()),
                    form,
                ]))
            }
            "@" => {
                reader.next(); // consume the at-sign
                let form = read_form(reader)?;
                Ok(Mal::List(vec![Mal::Symbol("deref".to_string()), form]))
            }
            "^" => {
                reader.next(); // consume the caret
                let form = read_form(reader)?;
                Ok(Mal::List(vec![Mal::Symbol("with-meta".to_string()), form]))
            }
            _ if token.starts_with("\"") => read_string(reader),
            _ => read_atom(reader),
        },
        None => Err("Unexpected EOF while reading form"),
    }
}

pub fn read_list(reader: &mut Reader) -> Result<Mal, &'static str> {
    let mut list = Vec::new();
    reader.next(); // consume '('
    while let Some(token) = reader.peek() {
        if token == ")" {
            reader.next(); // consume ')'
            return Ok(Mal::List(list));
        } else {
            list.push(read_form(reader)?);
        }
    }
    // When peek() returns None, we are at EOF with unbalanced parens
    Err("Unexpected EOF or unbalanced parentheses while reading list")
}

pub fn read_vector(reader: &mut Reader) -> Result<Mal, &'static str> {
    let mut list = Vec::new();
    reader.next(); // consume '['
    while let Some(token) = reader.peek() {
        if token == "]" {
            reader.next(); // consume ']'
            return Ok(Mal::Vector(list));
        } else {
            list.push(read_form(reader)?);
        }
    }
    // When peek() returns None, we are at EOF with unbalanced parens
    Err("Unexpected EOF or unbalanced parentheses while reading list")
}

pub fn read_hashmap(reader: &mut Reader) -> Result<Mal, &'static str> {
    let mut hashmap = HashMap::new();
    reader.next(); // consume '{'
    while let Some(token) = reader.peek() {
        if token == "}" {
            reader.next(); // consume '}'
            return Ok(Mal::HashMap(hashmap));
        } else {
            // read the key
            let key = read_form(reader)?;
            let key_str = match key {
                Mal::String(key_str) => key_str,
                Mal::Symbol(key_str) => key_str,
                _ => return Err("Expected string or symbol key in hashmap"),
            };
            // read the value
            let value = match read_form(reader) {
                Ok(val) => val,
                Err(_) => return Err("Expected value for key but got EOF or error"),
            };

            // println!("Inserting key: {}, value: {:?}", key_str, value);
            hashmap.insert(key_str, value);
        }
    }
    // When peek() returns None, we are at EOF with unbalanced parens
    Err("Unexpected EOF or unbalanced parentheses while reading list")
}

fn ends_with_unescaped_quote(token: &str) -> bool {
    let mut chars = token.chars().rev();
    if let Some(last_char) = chars.next() {
        if last_char == '"' {
            // Check if the quote is escaped
            let mut backslash_count = 0;
            for c in chars {
                if c == '\\' {
                    backslash_count += 1;
                } else {
                    break;
                }
            }
            // If the number of backslashes is even, the quote is not escaped
            return backslash_count % 2 == 0;
        }
    }
    false
}

pub fn read_string(reader: &mut Reader) -> Result<Mal, &'static str> {
    let token = reader.next().ok_or("Unexpected EOF while reading string")?;

    // Check if token ends with '"'
    if token.len() < 2 || !ends_with_unescaped_quote(&token) {
        return Err("Unexpected EOF while reading string: missing closing quote");
    }

    // Remove the surrounding quotes
    let content = &token[1..token.len() - 1];
    Ok(Mal::String(content.to_string()))
}

pub fn read_atom(reader: &mut Reader) -> Result<Mal, &'static str> {
    let token = reader.next().ok_or("Unexpected EOF while reading atom")?;

    // Try to parse token as an integer
    if let Ok(num) = token.parse::<i64>() {
        return Ok(Mal::Number(num));
    }

    if token.starts_with('\'') {}
    // Otherwise, treat as a symbol
    Ok(Mal::Symbol(token.clone()))
}
