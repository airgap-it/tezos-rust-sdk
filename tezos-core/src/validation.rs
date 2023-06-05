/// Validates that `value` matches Tezos string respecting escaping rules.
///
/// TODO: which unescaped special control sequences should be considered invalid?
/// see https://tezos.gitlab.io/alpha/michelson.html#constants (needs clarification)
pub fn is_str(value: &str) -> bool {
    let mut chars = value.chars();
    !chars.any(|c| match c {
        ' '..='~' => false,
        _ => true,
    })
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
    match chars.peek() {
        Some('0') => {
            chars.next();
            match chars.peek() {
                // once seen a '0', it must be followed by 'x' to build '0x' prefix
                Some('x') => {
                    chars.next();
                }
                _ => {
                    // not advancing cursor -> gets validated below
                }
            }
        }
        _ => {
            // not advancing cursor -> gets validated below or is an empty string (valid)
        }
    }
    !chars.any(|c| match c {
        '0'..='9' => false,
        'A'..='F' => false,
        'a'..='f' => false,
        _ => true,
    })
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
    !chars.any(|c| match c {
        '0'..='9' => false,
        _ => true,
    })
}

/// Validates that `value` matches equivalent regex `^[0-9]+$`.
pub fn is_uint(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    match chars.peek() {
        None => {
            // empty string is invalid!
            return false;
        }
        _ => {
            // not advancing cursor -> gets validated below
        }
    }
    !chars.any(|c| match c {
        '0'..='9' => false,
        _ => true,
    })
}
