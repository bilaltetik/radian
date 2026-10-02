// Radian Lang - symbol lexer'ı

use super::super::types::*;

/// Sıra önemsiz. Şart: her symbol'ün tüm önekleri de tabloda olmalı
/// ("..=" için ".." ve "."). Bunu `prefix_closed` testi doğrular.
pub const SYMBOLS: &[(&str, Symbol)] = &[
    // 3 karakter
    ("..=", Symbol::DotDotEq),
    // 2 karakter
    ("==", Symbol::EqEq), ("!=", Symbol::NotEq),
    ("<=", Symbol::LtEq), (">=", Symbol::GtEq),
    ("&&", Symbol::AndAnd), ("||", Symbol::OrOr),
    ("<<", Symbol::Shl), (">>", Symbol::Shr),
    ("+=", Symbol::PlusEq), ("-=", Symbol::MinusEq),
    ("*=", Symbol::StarEq), ("/=", Symbol::SlashEq),
    ("%=", Symbol::PercentEq),
    ("::", Symbol::ColonColon), ("..", Symbol::DotDot),
    ("->", Symbol::Arrow), ("=>", Symbol::FatArrow),
    // 1 karakter
    ("+", Symbol::Plus), ("-", Symbol::Minus), ("*", Symbol::Star),
    ("/", Symbol::Slash), ("%", Symbol::Percent),
    ("<", Symbol::Lt), (">", Symbol::Gt), ("!", Symbol::Bang),
    ("&", Symbol::Amp), ("|", Symbol::Pipe), ("^", Symbol::Caret),
    ("~", Symbol::Tilde), ("=", Symbol::Eq),
    ("(", Symbol::LParen), (")", Symbol::RParen),
    ("{", Symbol::LBrace), ("}", Symbol::RBrace),
    ("[", Symbol::LBracket), ("]", Symbol::RBracket),
    (",", Symbol::Comma), (";", Symbol::Semicolon),
    (":", Symbol::Colon), (".", Symbol::Dot),
    ("?", Symbol::Question), ("@", Symbol::At), ("#", Symbol::Hash),
];

/// Verilen metnin TAMAMI tabloda bir symbol mü.
pub fn symbol_from_str(s: &str) -> Option<Symbol> {
    SYMBOLS
        .iter()
        .find(|(text, _)| *text == s)
        .map(|(_, sym)| *sym)
}

/// Tek karakterlik peek: bu karakter bir symbol'ün başlangıcı mı?
pub fn match_symbol(c: char) -> Option<Symbol> {
    let mut buf = [0u8; 4];
    symbol_from_str(c.encode_utf8(&mut buf))
}

/// Girdinin başından en uzun symbol'ü okur: (symbol, byte uzunluğu).
/// Her adımda bir karakter daha ekler; sonuç hâlâ tanımlı bir symbol
/// olduğu sürece uzar, tanımsızlaşınca durur.
pub fn lex_symbol(input: &str) -> Option<(Symbol, usize)> {
    let first = input.chars().next()?;
    let mut best = (match_symbol(first)?, first.len_utf8());

    for (i, c) in input.char_indices().skip(1) {
        let end = i + c.len_utf8();
        match symbol_from_str(&input[..end]) {
            Some(sym) => best = (sym, end),
            None => break,
        }
    }
    Some(best)
}