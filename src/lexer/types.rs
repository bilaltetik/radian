

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerAffix {
    None,
    Binary,
    Octal,
    Hexadecimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecimalAffix {
    None,
    Scientific,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    Integer(IntegerAffix),
    Decimal(DecimalAffix),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    String,
    Char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordDomain {
    Var,
    As,
}

impl KeywordDomain {
    /// Identifier olarak okunan kelime keyword mü diye bakar.
    pub fn from_str(s: &str) -> Option<KeywordDomain> {
        Some(match s {
            "var" => KeywordDomain::Var,
            "As"  => KeywordDomain::As,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordKind {
    //None, //hata
    Identifier,
    Keyword(KeywordDomain),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolDomain {
    // aritmetik
    Plus, 
    Minus,

    Eq,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    TokenNumber(NumberKind),
    TokenLiteral(LiteralKind),
    TokenWord(WordKind),
    TokenSymbol(SymbolDomain),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,

    pub lexeme: String,
    pub span: Span,
}