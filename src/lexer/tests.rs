// Radian Lang - lexer testleri

use super::types::*;
use super::utils::number::*;
use super::utils::string::*;
use super::utils::symbol::*;

// Tablodaki her symbol'ün tüm önekleri de tabloda olmalı,
// yoksa lex_symbol uzun symbol'e ulaşamaz.
#[test]
fn prefix_closed() {
    for (text, _) in TOKEN_SYMBOL {
        for end in 1..=text.len() {
            assert!(
                symbol_from_str(&text[..end]).is_some(),
                "'{}' tabloda ama öneki '{}' yok",
                text,
                &text[..end]
            );
        }
    }
}

#[test]
fn no_duplicate_texts() {
    for (i, (a, _)) in TOKEN_SYMBOL.iter().enumerate() {
        for (b, _) in &TOKEN_SYMBOL[i + 1..] {
            assert_ne!(a, b, "'{}' tabloda iki kez var", a);
        }
    }
}

// Her symbol kendi metninden aynen geri okunabilmeli.
#[test]
fn every_symbol_roundtrips() {
    for (text, sym) in TOKEN_SYMBOL {
        assert_eq!(lex_symbol(text), Some((*sym, text.len())), "'{}'", text);
    }
}

/*
#[test]
fn single_char_peek() {
    assert_eq!(match_symbol('+'), Some(SymbolDomain::Plus));
    assert_eq!(match_symbol('.'), Some(SymbolDomain::Dot));
    assert_eq!(match_symbol('a'), None);
    assert_eq!(match_symbol('5'), None);
    assert_eq!(match_symbol('ş'), None);
}

#[test]
fn maximal_munch() {
    assert_eq!(lex_symbol("..=5"), Some((SymbolDomain::DotDotEq, 3)));
    assert_eq!(lex_symbol("..5"), Some((SymbolDomain::DotDot, 2)));
    assert_eq!(lex_symbol(".5"), Some((SymbolDomain::Dot, 1)));
    assert_eq!(lex_symbol("<<1"), Some((SymbolDomain::Shl, 2)));
    assert_eq!(lex_symbol("->x"), Some((SymbolDomain::Arrow, 2)));
}

// Peş peşe symbol'ler tek token'a yapışmamalı.
#[test]
fn stops_when_undefined() {
    assert_eq!(lex_symbol("=-1"), Some((SymbolDomain::Eq, 1)));
    assert_eq!(lex_symbol("()"), Some((SymbolDomain::LParen, 1)));
    assert_eq!(lex_symbol("<<="), Some((SymbolDomain::Shl, 2)));
    assert_eq!(lex_symbol("+ş"), Some((SymbolDomain::Plus, 1)));
}

*/

#[test]
fn not_a_symbol() {
    assert_eq!(lex_symbol(""), None);
    assert_eq!(lex_symbol("a+"), None);
    assert_eq!(lex_symbol("$"), None);
    assert_eq!(lex_symbol("ş"), None);
}

#[test]
fn keyword_lookup() {
    assert_eq!(KeywordDomain::from_str("let"), Some(KeywordDomain::Var));
    assert_eq!(KeywordDomain::from_str("letter"), None);
}

// ---------- sayılar ----------

fn int(a: IntegerAffix, len: usize) -> Result<(NumberKind, usize), String> {
    Ok((NumberKind::Integer(a), len))
}
fn dec(a: DecimalAffix, len: usize) -> Result<(NumberKind, usize), String> {
    Ok((NumberKind::Decimal(a), len))
}

#[test]
fn number_integers() {
    assert_eq!(lex_number("0"), int(IntegerAffix::None, 1));
    assert_eq!(lex_number("42"), int(IntegerAffix::None, 2));
    assert_eq!(lex_number("0543"), int(IntegerAffix::None, 4)); // baştaki sıfır serbest
    assert_eq!(lex_number("12u8"), int(IntegerAffix::None, 4));
    assert_eq!(lex_number("1+2"), int(IntegerAffix::None, 1));
}

#[test]
fn number_prefixed() {
    assert_eq!(lex_number("0b10"), int(IntegerAffix::Binary, 4));
    assert_eq!(lex_number("0o17"), int(IntegerAffix::Octal, 4));
    assert_eq!(lex_number("0x1F"), int(IntegerAffix::Hexadecimal, 4));
    assert_eq!(lex_number("0x1Fu8"), int(IntegerAffix::Hexadecimal, 6));
    assert_eq!(lex_number("0xFE"), int(IntegerAffix::Hexadecimal, 4)); // 'e' üs değil
}

#[test]
fn number_decimals() {
    assert_eq!(lex_number("56.48"), dec(DecimalAffix::None, 5));
    assert_eq!(lex_number("0.5"), dec(DecimalAffix::None, 3));
    assert_eq!(lex_number("1.5f32"), dec(DecimalAffix::None, 6));
    assert_eq!(lex_number("1e10"), dec(DecimalAffix::Scientific, 4));
    assert_eq!(lex_number("1.5e-3"), dec(DecimalAffix::Scientific, 6));
    assert_eq!(lex_number("0e3"), dec(DecimalAffix::Scientific, 3));
}

// '.' sayıya ait değilse tüketilmemeli
#[test]
fn number_dot_not_consumed() {
    assert_eq!(lex_number("1..5"), int(IntegerAffix::None, 1));
    assert_eq!(lex_number("1.method()"), int(IntegerAffix::None, 1));
    assert_eq!(lex_number("1."), int(IntegerAffix::None, 1));
    assert_eq!(lex_number("1.5.3"), dec(DecimalAffix::None, 3)); // gerisi ".3": lexer'ın işi değil
}

#[test]
fn number_errors() {
    assert!(lex_number("").is_err());
    assert!(lex_number(".5").is_err()); // sayı rakamla başlar
    assert!(lex_number("-5").is_err()); // işaret Symbol'dür
    assert!(lex_number("0x").is_err());
    assert!(lex_number("0b102").is_err());
    assert!(lex_number("12abc").is_err());
    assert!(lex_number("1e").is_err());
    assert!(lex_number("1f32").is_err()); // grammar'da f suffix'i sadece ondalıkta
}

#[test]
fn number_start_peek() {
    assert!(match_number('7'));
    assert!(!match_number('.'));
    assert!(!match_number('a'));
}

#[test]
fn string_start_peek() {
    assert_eq!(match_string('"'), Some(LiteralKind::String));
    assert_eq!(match_string('\''), Some(LiteralKind::Char));
    assert_eq!(match_string('a'), None);
}

#[test]
fn string_basic() {
    assert_eq!(lex_string("\"abc\" x"), Ok((LiteralKind::String, 5)));
    assert_eq!(lex_string("\"\""), Ok((LiteralKind::String, 2)));
    assert_eq!(lex_string("\"it's\""), Ok((LiteralKind::String, 6))); // ' kapatmaz
    assert_eq!(lex_string("\"é\""), Ok((LiteralKind::String, 4))); // byte uzunluğu
}

#[test]
fn string_escapes() {
    assert_eq!(lex_string("\"a\\\"b\""), Ok((LiteralKind::String, 6))); // \" kapatmaz
    assert_eq!(lex_string("\"\\x41\""), Ok((LiteralKind::String, 6)));
    assert_eq!(lex_string("\"\\u{1F600}\""), Ok((LiteralKind::String, 11)));
}

#[test]
fn char_literals() {
    assert_eq!(lex_string("'a'"), Ok((LiteralKind::Char, 3)));
    assert_eq!(lex_string("'\\n'"), Ok((LiteralKind::Char, 4)));
    assert_eq!(lex_string("'\\x41'"), Ok((LiteralKind::Char, 6)));
    assert!(lex_string("''").is_err());
    assert!(lex_string("'ab'").is_err());
}

#[test]
fn string_errors() {
    assert!(lex_string("").is_err());
    assert!(lex_string("abc").is_err());
    assert!(lex_string("\"abc").is_err()); // kapanmamış
    assert!(lex_string("\"a\nb\"").is_err()); // newline
    assert!(lex_string("\"a\\").is_err()); // escape'te biten girdi
    assert!(lex_string("\"\\q\"").is_err()); // bilinmeyen escape
    assert!(lex_string("\"\\x4\"").is_err()); // eksik hex
}