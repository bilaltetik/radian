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
pub enum Keyword {
    Let,
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

impl Keyword {
    /// Identifier olarak okunan kelime keyword mü diye bakar.
    pub fn from_str(s: &str) -> Option<Keyword> {
        Some(match s {
            "let" => Keyword::Let,
            "mut" => Keyword::Mut,
            "const" => Keyword::Const,
            "fn" => Keyword::Fn,
            "return" => Keyword::Return,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "for" => Keyword::For,
            "in" => Keyword::In,
            "loop" => Keyword::Loop,
            "break" => Keyword::Break,
            "continue" => Keyword::Continue,
            "struct" => Keyword::Struct,
            "enum" => Keyword::Enum,
            "impl" => Keyword::Impl,
            "true" => Keyword::True,
            "false" => Keyword::False,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordKind {
    None, //hata
    Identifier,
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
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
    Number(NumberKind),
    Literal(LiteralKind),
    Word(WordKind),
    Symbol(Symbol),
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