// Radian Lang - sayı lexer'ı

use super::super::types::*;

const INT_SUFFIXES: &[&str] = &["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"];
const FLOAT_SUFFIXES: &[&str] = &["f32", "f64"];

/// Ana döngüdeki ucuz peek: bu karakter bir sayının başlangıcı mı?
/// (Kind'i burada bilemeyiz: "1" mi "1.5" mi, ancak okuyunca belli olur.)
pub fn match_number(c: char) -> bool {
    c.is_ascii_digit()
}

/// Verilen tabanda bu byte bir rakam mı.
fn is_digit_of(affix: IntegerAffix, c: u8) -> bool {
    match affix {
        IntegerAffix::None => c.is_ascii_digit(),
        IntegerAffix::Binary => matches!(c, b'0' | b'1'),
        IntegerAffix::Octal => matches!(c, b'0'..=b'7'),
        IntegerAffix::Hexadecimal => c.is_ascii_hexdigit(),
    }
}

/// i'den itibaren rakam olan byte'ları geçer, duracağı konumu döndürür.
fn skip_digits(b: &[u8], mut i: usize, affix: IntegerAffix) -> usize {
    while i < b.len() && is_digit_of(affix, b[i]) {
        i += 1;
    }
    i
}

/// i konumunda listedeki suffix'lerden biri varsa sonrasına, yoksa i'ye döner.
fn take_suffix(input: &str, i: usize, suffixes: &[&str]) -> usize {
    let rest = &input[i..];
    suffixes
        .iter()
        .find(|s| rest.starts_with(**s))
        .map_or(i, |s| i + s.len())
}

/// Sayıdan hemen sonra harf/rakam/_ geliyorsa hata: "0b102", "12abc", "1e".
fn check_end(input: &str, end: usize) -> Result<(), String> {
    match input[end..].chars().next() {
        Some(c) if c.is_alphanumeric() || c == '_' => {
            Err(format!("unexpected '{}' after number literal", c))
        }
        _ => Ok(()),
    }
}

/// Girdinin başındaki sayıyı okur: (kind, byte uzunluğu).
/// Sayı boyunca sadece ASCII byte'lar ilerlendiği için dilimleme güvenli.
pub fn lex_number(input: &str) -> Result<(NumberKind, usize), String> {
    let b = input.as_bytes();
    if !b.first().map_or(false, |c| c.is_ascii_digit()) {
        return Err(String::from("Not a number literal"));
    }

    // 1) Prefix'li tamsayı: 0b / 0o / 0x
    if b[0] == b'0' {
        let affix = match b.get(1).copied() {
            Some(b'b') | Some(b'B') => Some(IntegerAffix::Binary),
            Some(b'o') | Some(b'O') => Some(IntegerAffix::Octal),
            Some(b'x') | Some(b'X') => Some(IntegerAffix::Hexadecimal),
            _ => None,
        };
        if let Some(affix) = affix {
            let start = 2;
            let end = skip_digits(b, start, affix);
            if end == start {
                return Err(format!("expected digits after '{}'", &input[..2]));
            }
            let end = take_suffix(input, end, INT_SUFFIXES);
            check_end(input, end)?;
            return Ok((NumberKind::Integer(affix), end));
        }
    }

    // 2) Ondalık taban: tam kısım (baştaki sıfırlar serbest: 0543)
    let mut i = skip_digits(b, 0, IntegerAffix::None);
    let mut is_decimal = false;
    let mut scientific = false;

    // Kesir: '.' VE ardından rakam varsa (1..5 ve 1.method() bozulmaz)
    if b.get(i).copied() == Some(b'.') && b.get(i + 1).map_or(false, |c| c.is_ascii_digit()) {
        i = skip_digits(b, i + 1, IntegerAffix::None);
        is_decimal = true;
    }

    // Üs: e/E, isteğe bağlı işaret, en az bir rakam
    if matches!(b.get(i).copied(), Some(b'e') | Some(b'E')) {
        let mut j = i + 1;
        if matches!(b.get(j).copied(), Some(b'+') | Some(b'-')) {
            j += 1;
        }
        let end = skip_digits(b, j, IntegerAffix::None);
        if end > j {
            i = end;
            is_decimal = true;
            scientific = true;
        }
        // rakam yoksa 'e' tüketilmez, check_end hata verir
    }

    let (kind, suffixes) = if is_decimal {
        let affix = if scientific { DecimalAffix::Scientific } else { DecimalAffix::None };
        (NumberKind::Decimal(affix), FLOAT_SUFFIXES)
    } else {
        (NumberKind::Integer(IntegerAffix::None), INT_SUFFIXES)
    };

    let end = take_suffix(input, i, suffixes);
    check_end(input, end)?;
    Ok((kind, end))
}