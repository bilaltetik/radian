use super::super::types::*;


pub const TOKEN_SYMBOL: &[(&str, SymbolDomain)] = &[
    // 3 karakter
    ("..=", SymbolDomain::DotDotEq),
    // 2 karakter
    ("==", SymbolDomain::EqEq), ("!=", SymbolDomain::NotEq),
    ("<=", SymbolDomain::LtEq), (">=", SymbolDomain::GtEq),
    ("&&", SymbolDomain::AndAnd), ("||", SymbolDomain::OrOr),
    ("<<", SymbolDomain::Shl), (">>", SymbolDomain::Shr),
    ("+=", SymbolDomain::PlusEq), ("-=", SymbolDomain::MinusEq),
    ("*=", SymbolDomain::StarEq), ("/=", SymbolDomain::SlashEq),
    ("%=", SymbolDomain::PercentEq),
    ("::", SymbolDomain::ColonColon), ("..", SymbolDomain::DotDot),
    ("->", SymbolDomain::Arrow), ("=>", SymbolDomain::FatArrow),
    // 1 karakter
    ("+", SymbolDomain::Plus), ("-", SymbolDomain::Minus), ("*", SymbolDomain::Star),
    ("/", SymbolDomain::Slash), ("%", SymbolDomain::Percent),
    ("<", SymbolDomain::Lt), (">", SymbolDomain::Gt), ("!", SymbolDomain::Bang),
    ("&", SymbolDomain::Amp), ("|", SymbolDomain::Pipe), ("^", SymbolDomain::Caret),
    ("~", SymbolDomain::Tilde), ("=", SymbolDomain::Eq),
    ("(", SymbolDomain::LParen), (")", SymbolDomain::RParen),
    ("{", SymbolDomain::LBrace), ("}", SymbolDomain::RBrace),
    ("[", SymbolDomain::LBracket), ("]", SymbolDomain::RBracket),
    (",", SymbolDomain::Comma), (";", SymbolDomain::Semicolon),
    (":", SymbolDomain::Colon), (".", SymbolDomain::Dot),
    ("?", SymbolDomain::Question), ("@", SymbolDomain::At), ("#", SymbolDomain::Hash),
];

pub fn symbol_from_str(s: &str) -> Option<SymbolDomain> {
    TOKEN_SYMBOL
        .iter()
        .find(|(text, _)| *text == s)
        .map(|(_, sym)| *sym)
}

pub fn match_symbol(c: char) -> Option<SymbolDomain> {
    let mut buf = [0u8; 4];
    symbol_from_str(c.encode_utf8(&mut buf))
}


pub fn lex_symbol(input: &str) -> Option<(SymbolDomain, usize)> {
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