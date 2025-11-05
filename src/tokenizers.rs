// TODO: Define your Token enum here
//
// Hint: You need variants for:
// LeftBrace, RightBrace, LeftBracket, RightBracket, Comma, Colon
// String(String), Number(f64), Boolean(bool), Null
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,   
}
// TODO: Implement your tokenize function here
pub fn tokenize(input: &str) -> Vec<Token> {
     // Your code goes here
    let mut it = input.chars().peekable();
    let mut tokens = Vec::new();

    // &str is "string slice" - a view intro string data
    // It doesn't own the data, just borrows it (pass as value?)
    while let Some(&c) = it.peek() {
        match c {
            c if c.is_ascii_digit() => {
                let mut buf = String::new();
                while let Some(&d) = it.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        buf.push(d);
                        it.next();
                    } else {
                        break;
                    }
                }
                if let Ok(v) = buf.parse::<f64>() {
                    tokens.push(Token::Number(v));
                }
            },
            // Keywords: true, false, null
            c if c.is_ascii_alphabetic() => {
                let mut word = String::new();
                while let Some(&w) = it.peek() {
                    if w.is_ascii_alphabetic() {
                        word.push(w);
                        it.next();
                    } else {
                        break;
                    }
                }
                match word.as_str() {
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    "null" => tokens.push(Token::Null),
                    _ => {
                        // Ignore rest of unknown tokens for the time being
                    }
                }
            },
            '"' => {
                it.next();      // Consume opening "
                let mut buf = String::new();

                while let Some(ch)= it.next() {                    
                    if ch == '"' {
                        break;  // Ending "
                    } else {
                        buf.push(ch);
                    }
                }
                tokens.push(Token::String(buf));
            },
            // JSON Puntuation
            '{' => {tokens.push(Token::LeftBrace); it.next();},
            '}' => {tokens.push(Token::RightBrace); it.next();},
            '[' => {tokens.push(Token::LeftBracket); it.next();},
            ']' => {tokens.push(Token::RightBracket); it.next();},
            ',' => {tokens.push(Token::Comma); it.next();},
            ':' => {tokens.push(Token::Colon); it.next();},
            _ => { it.next(); } // Skip other characters for now
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_braces() {
        let tokens = tokenize("{}");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
    }

    #[test]
    fn test_simple_string() {
        let tokens = tokenize(r#""hello""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }

    #[test]
    fn test_number() {
        let tokens = tokenize("42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
    }

    #[test]
    fn test_tokenize_string() {
        let tokens = tokenize(r#""hello world""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
    }

    #[test]
    fn test_boolean_and_null() {
        let tokens = tokenize("true false null");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
    }

    #[test]
    fn test_simple_object() {
        let tokens = tokenize(r#"{"name": "Alice"}"#);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("name".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::String("Alice".to_string()));
        assert_eq!(tokens[4], Token::RightBrace);
    }

    #[test]
    fn test_multiple_values() {
        let tokens = tokenize(r#"{"age": 30, "active": true}"#);
        // Verify we have the right tokens
        assert!(tokens.contains(&Token::String("age".to_string())));
        assert!(tokens.contains(&Token::Number(30.0)));
        assert!(tokens.contains(&Token::Comma));
        assert!(tokens.contains(&Token::String("active".to_string())));
        assert!(tokens.contains(&Token::Boolean(true)));
    }
}
