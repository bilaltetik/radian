use super::super::types::*;

pub fn match_string(c: char) -> Option<LiteralKind> {
    match c {
        '"' => Some(LiteralKind::String),
        '\'' => Some(LiteralKind::Char),
        _ => None,
    }
}

pub fn lex_string(input: &str) -> Result<(LiteralKind, usize), String> {
    let mut chars = input.char_indices();

    let open = match chars.next() {
        Some((_, c)) => c,
        None => return Err(String::from("Not a string literal")),
    };
    let kind = match match_string(open) {
        Some(k) => k,
        None => return Err(String::from("Not a string literal")),
    };

    let mut count = 0;

    while let Some((i, c)) = chars.next() {
        if c == open {
            if kind == LiteralKind::Char && count != 1 {
                return Err(String::from("char literal must contain exactly one character"));
            }
            return Ok((kind, i + c.len_utf8()));
        }
        match c {
            '\n' => return Err(String::from("Unterminated literal (newline)")),
            '\\' => {
                match chars.next().map(|(_, e)| e) {
                    Some('n' | 't' | 'r' | '0' | '\\' | '\'' | '"') => {}
                    Some('x') => {
                        for _ in 0..2 {
                            match chars.next() {
                                Some((_, h)) if h.is_ascii_hexdigit() => {}
                                _ => return Err(String::from("invalid \\x escape")),
                            }
                        }
                    }
                    Some('u') => {
                        if chars.next().map(|(_, e)| e) != Some('{') {
                            return Err(String::from("invalid \\u escape"));
                        }
                        let mut n = 0;
                        loop {
                            match chars.next() {
                                Some((_, '}')) if n > 0 => break,
                                Some((_, h)) if h.is_ascii_hexdigit() => n += 1,
                                _ => return Err(String::from("invalid \\u escape")),
                            }
                        }
                    }
                    _ => return Err(String::from("unknown escape sequence")),
                }
                count += 1;
            }
            _ => count += 1,
        }
    }
    Err(String::from("Unterminated literal"))
}