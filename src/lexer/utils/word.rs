use super::super::types::*;


pub fn match_word(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_word_cont(c: char) -> bool {
    match_word(c) || c.is_ascii_digit()
}

pub fn lex_word(input: &str) -> Option<(WordKind, usize)> {
    if !input.chars().next().map_or(false, match_word) {
        return None;
    }

    let len = input
        .char_indices()
        .find(|&(_, c)| !is_word_cont(c))
        .map_or(input.len(), |(i, _)| i);

    let kind = match KeywordDomain::from_str(&input[..len]) {
        Some(k) => WordKind::Keyword(k),
        None => WordKind::Identifier,
    };
    Some((kind, len))
}