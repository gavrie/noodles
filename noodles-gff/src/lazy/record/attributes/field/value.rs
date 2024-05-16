/// A raw GFF record attributes field value.
#[derive(Debug, Eq, PartialEq)]
pub enum Value<'a> {
    /// A string.
    String(&'a str),
    /// An array.
    Array(&'a str),
}

pub(super) fn parse_value(s: &str) -> Value<'_> {
    if is_array(s) {
        Value::Array(s)
    } else {
        Value::String(s)
    }
}

fn is_array(s: &str) -> bool {
    const SEPARATOR: char = ',';
    s.contains(SEPARATOR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("ndls"), Value::String("ndls"));
        assert_eq!(parse_value("nd,ls"), Value::Array("nd,ls"));
    }

    #[test]
    fn test_is_array() {
        assert!(is_array("nd,ls"));
        assert!(!is_array("ndls"));
    }
}
