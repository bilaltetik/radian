use crate::lexer::utils::symbol;

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
    Mut,
    Const,
    Fn,
    Return,
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Struct,
    Enum,
    Impl,
    True,
    False,
}

impl KeywordDomain {
    /// Identifier olarak okunan kelime keyword mü diye bakar.
    pub fn from_str(s: &str) -> Option<KeywordDomain> {
        Some(match s {
            "var" => KeywordDomain::Var,
            "as" => KeywordDomain::As,
            "mut" => KeywordDomain::Mut,
            "const" => KeywordDomain::Const,
            "fn" => KeywordDomain::Fn,
            "return" => KeywordDomain::Return,
            "if" => KeywordDomain::If,
            "else" => KeywordDomain::Else,
            "while" => KeywordDomain::While,
            "for" => KeywordDomain::For,
            "in" => KeywordDomain::In,
            "loop" => KeywordDomain::Loop,
            "break" => KeywordDomain::Break,
            "continue" => KeywordDomain::Continue,
            "struct" => KeywordDomain::Struct,
            "enum" => KeywordDomain::Enum,
            "impl" => KeywordDomain::Impl,
            "true" => KeywordDomain::True,
            "false" => KeywordDomain::False,
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
    Plus, Minus, Star, Slash, Percent,
    // karşılaştırma
    EqEq, NotEq, Lt, LtEq, Gt, GtEq,
    // mantıksal
    AndAnd, OrOr, Bang,
    // bit
    Amp, Pipe, Caret, Tilde, Shl, Shr,
    // atama
    Eq, PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
    // gruplama
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    // noktalama
    Comma, Semicolon, Colon, ColonColon, Dot, DotDot, DotDotEq,
    Arrow, FatArrow, Question, At, Hash,
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