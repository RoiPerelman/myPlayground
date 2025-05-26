pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '~' => {
                if let Some(&'@') = chars.peek() {
                    chars.next(); // consume '@'
                    tokens.push("~@".to_string());
                } else {
                    tokens.push("~".to_string());
                }
            }
            '[' | ']' | '{' | '}' | '(' | ')' | '\'' | '`' | '^' | '@' => {
                tokens.push(c.to_string());
            }
            '"' => {
                // handle string tokenization (example)
                let mut string_token = String::new();
                string_token.push(c);
                while let Some(next_char) = chars.next() {
                    string_token.push(next_char);
                    if next_char == '"' {
                        break;
                    }
                    if next_char == '\\' {
                        // escape next char if exists
                        if let Some(esc) = chars.next() {
                            string_token.push(esc);
                        }
                    }
                }
                tokens.push(string_token);
            }
            ';' => {
                // comment token: consume until newline or end
                let mut comment = String::new();
                comment.push(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char == '\n' {
                        break;
                    }
                    comment.push(chars.next().unwrap());
                }
                tokens.push(comment);
            }
            c if c.is_whitespace() || c == ',' => {
                // skip whitespace and commas
            }
            _ => {
                // capture symbols, numbers, true, false, nil, etc.
                let mut symbol = String::new();
                symbol.push(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_whitespace() || "[]{}'`~^@,\";".contains(next_char) {
                        break;
                    }
                    symbol.push(chars.next().unwrap());
                }
                tokens.push(symbol);
            }
        }
    }
    return tokens;
}
