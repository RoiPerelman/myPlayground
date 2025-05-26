use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    let pattern = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
    let re = Regex::new(pattern).unwrap();

    let mut tokens = Vec::new();

    for cap in re.captures_iter(input) {
        let token = &cap[1];
        if !token.is_empty() {
            tokens.push(token.to_string());
        }
    }

    tokens
}
