/// Validates that `value` matches Tezos string respecting escaping rules.
///
/// TODO: which unescaped special control sequences should be considered invalid?
/// see https://tezos.gitlab.io/alpha/michelson.html#constants (needs clarification)
pub fn is_str(value: &str) -> bool {
    let mut chars = value.chars();
    !chars.any(|c| !matches!(c, ' '..='~'))
}

/// Validates that `value` matches equivalent regex `^(0x)?([0-9a-fA-F]{2})*$`.
///
/// TODO: which unescaped special control sequences should be considered invalid?
/// see https://tezos.gitlab.io/alpha/michelson.html#constants (needs clarification)
pub fn is_hex_str(value: &str) -> bool {
    if value.len() % 2 != 0 {
        return false;
    }
    let mut chars = value.chars().peekable();

    if let Some('0') = chars.peek() {
        chars.next();
        if let Some('x') = chars.peek() {
            chars.next();
        }
    }

    !chars.any(|c| !matches!(c, '0'..='9' | 'A'..='F' | 'a'..='f'))
}

/// Validates that `value` matches equivalent regex `^-?[0-9]+$`.
pub fn is_int(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    match chars.peek() {
        None => {
            // no element!
            return false;
        }
        Some('-') => {
            chars.next();
        }
        _ => {
            // not advancing cursor -> gets validated below
        }
    }
    chars.all(|c| c.is_ascii_digit())
}

/// Validates that `value` matches equivalent regex `^[0-9]+$`.
pub fn is_uint(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    if chars.peek().is_none() {
        // empty string is invalid!
        return false;
    }
    chars.all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod test {
    use crate::validation::{is_int, is_str, is_uint};

    #[test]
    fn test_is_7bit_ascii() {
        assert!(is_str("123456789"));
        assert!(is_str("ABCabc!"));
        assert!(is_str("-12"));
        assert!(!is_str("\n"));
    }

    #[test]
    fn test_is_int() {
        assert!(is_int("123456789"));
        assert!(is_int("0"));
        assert!(is_int("-12"));
        assert!(!is_int("A"));
    }

    #[test]
    fn test_is_uint() {
        assert!(is_uint("123456789"));
        assert!(is_uint("0"));
        assert!(!is_uint("-12"));
        assert!(!is_uint("A"));
    }
}
