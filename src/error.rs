// Week 2: Custom error type for JSON parsing
use std::fmt;

// TODO: Define your JsonError enum here
// Hint: You need variants for:
// - UnexpectedToken { expected: String, found: String, position: usize }
// - UnexpectedEndOfInput { expected: String, position: usize }
// - InvalidNumber { value: String, position: usize }

#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: String,
        position: usize,
    },
    InvalidNumber {
        value: String,
        position: usize,
    },
}
// TODO: Implement Display trait
// impl fmt::Display for JsonError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Your code here
//     }
// }
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken { expected, found, position } => write!(f, "Unexpected token: expected {}, found {}, at position {}", expected, found, position),
            JsonError::UnexpectedEndOfInput { expected, position } => write!(f, "Unexpected end of input: expected {}, at position {}", expected, position),
            JsonError::InvalidNumber { value, position } => write!(f, "Invalid number: {}, at position {}", value, position),
        }
    }
}

// TODO: Implement Error trait
// impl std::error::Error for JsonError {}
impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

// Copy these tests as-is:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "@".to_string(),
            position: 5,
        };

        // Error should be Debug-printable
        assert!(format!("{:?}", error).contains("UnexpectedToken"));
    }

    #[test]
    fn test_error_display() {
        let error = JsonError::UnexpectedToken {
            expected: "valid JSON".to_string(),
            found: "@".to_string(),
            position: 0,
        };

        let message = format!("{}", error);
        assert!(message.contains("position 0"));
        assert!(message.contains("valid JSON"));
        assert!(message.contains("@"));
    }

    #[test]
    fn test_error_variants() {
        let token_error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "x".to_string(),
            position: 3,
        };

        let eof_error = JsonError::UnexpectedEndOfInput {
            expected: "closing quote".to_string(),
            position: 10,
        };

        let num_error = JsonError::InvalidNumber {
            value: "12.34.56".to_string(),
            position: 0,
        };

        // All variants should be Debug-printable
        format!("{:?}", token_error);
        format!("{:?}", eof_error);
        format!("{:?}", num_error);
    }
}