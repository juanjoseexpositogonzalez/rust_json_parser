// Week 2: JsonValue enum to represent parsed JSON values
// Week 2 focuses on basic types only - no collections yet

// TODO: Define your JsonValue enum here
// Hint: You need variants for:
// - Null
// - Boolean(bool)
// - Number(f64)
// - String(String)
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
}

// TODO: Implement helper methods
impl JsonValue {
  pub fn is_null(&self) -> bool { 
    match self {
        JsonValue::Null => return true,
        _ => false,
    }
  }
  pub fn as_str(&self) -> Option<&str> { 
    match self {
        JsonValue::String(s) => Some(s),
        _ => return None,
    }
  }
  pub fn as_f64(&self) -> Option<f64> { 
    match self {
        JsonValue::Number(n) => Some(*n),
        _ => return None,
    }
  }
  pub fn as_bool(&self) -> Option<bool> { 
    match self {
        JsonValue::Boolean(b) => Some(*b),
        _ => None,
    }
  }
}

// Copy these tests as-is:
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_json_value_creation() {
    let null_val = JsonValue::Null;
    let bool_val = JsonValue::Boolean(true);
    let num_val = JsonValue::Number(42.5);
    let str_val = JsonValue::String("hello".to_string());

    assert!(null_val.is_null());
    assert_eq!(bool_val.as_bool(), Some(true));
    assert_eq!(num_val.as_f64(), Some(42.5));
    assert_eq!(str_val.as_str(), Some("hello"));
   }

  #[test]
  fn test_json_value_accessors() {
    let value = JsonValue::String("test".to_string());
    assert_eq!(value.as_str(), Some("test"));
    assert_eq!(value.as_f64(), None);
    assert_eq!(value.as_bool(), None);
    assert!(!value.is_null());

    let value = JsonValue::Number(42.0);
    assert_eq!(value.as_f64(), Some(42.0));
    assert_eq!(value.as_str(), None);

    let value = JsonValue::Boolean(true);
    assert_eq!(value.as_bool(), Some(true));

    let value = JsonValue::Null;
    assert!(value.is_null());
   }

  #[test]
  fn test_json_value_equality() {
    assert_eq!(JsonValue::Null, JsonValue::Null);
    assert_eq!(JsonValue::Boolean(true), JsonValue::Boolean(true));
    assert_eq!(JsonValue::Number(42.0), JsonValue::Number(42.0));
    assert_eq!(
      JsonValue::String("test".to_string()),
      JsonValue::String("test".to_string())
     );

    assert_ne!(JsonValue::Null, JsonValue::Boolean(false));
    assert_ne!(JsonValue::Number(1.0), JsonValue::Number(2.0));
   }
}
