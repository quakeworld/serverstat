pub fn tokenize(value: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut in_quote = false;
    let mut current_token = "".to_string();

    for c in value.chars() {
        match c {
            '"' => in_quote = !in_quote,
            ' ' => {
                if in_quote {
                    current_token.push(c);
                } else {
                    tokens.push(current_token);
                    current_token = "".to_string();
                }
            }
            _ => current_token.push(c),
        }
    }

    tokens.push(current_token);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(r#"qtv 1 "zasadzka Qtv (2)" "2@zasadzka.pl:28000" 2"#),
            vec![
                "qtv".to_string(),
                "1".to_string(),
                "zasadzka Qtv (2)".to_string(),
                "2@zasadzka.pl:28000".to_string(),
                "2".to_string(),
            ]
        );
        assert_eq!(
            tokenize(r#"24 S 0 667 "[ServeMe]" "" 12 11 "lqwc" """#),
            vec![
                "24".to_string(),
                "S".to_string(),
                "0".to_string(),
                "667".to_string(),
                "[ServeMe]".to_string(),
                "".to_string(),
                "12".to_string(),
                "11".to_string(),
                "lqwc".to_string(),
                "".to_string(),
            ]
        );
    }
}
