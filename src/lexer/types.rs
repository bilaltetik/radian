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
pub enum Keyword {
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

impl Keyword {
    /// Identifier olarak okunan kelime keyword mü diye bakar.
    pub fn from_str(s: &str) -> Option<Keyword> {
        Some(match s {
            "var" => Keyword::Var,
            "as" => Keyword::As,
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
    //None, //hata
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


pub enum Operation{
    Sym(Symbol),
    Key(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
    None, // zincirlenemez (a < b < c gibi)
}


impl Operation {
    pub fn precedence(&self) -> Option<(u8, Assoc)> {
        use Assoc::*;
        match self {
            Operation::Sym(s) => match s {
                // Erişim / postfix
                Symbol::Dot | Symbol::Arrow => Some((14, Left)),
                Symbol::Question => Some((13, Left)),

                // Prefix unary
                Symbol::Bang | Symbol::Tilde => Some((12, Right)),

                // Çarpma grubu
                Symbol::Star | Symbol::Slash | Symbol::Percent => Some((11, Left)),

                // Toplama grubu
                Symbol::Plus | Symbol::Minus => Some((10, Left)),

                // Kaydırma
                Symbol::Shl | Symbol::Shr => Some((9, Left)),

                // Bit işlemleri
                Symbol::Amp => Some((8, Left)),
                Symbol::Caret => Some((7, Left)),
                Symbol::Pipe => Some((6, Left)),

                // Karşılaştırma
                Symbol::Lt | Symbol::LtEq | Symbol::Gt | Symbol::GtEq => Some((5, None)),
                Symbol::EqEq | Symbol::NotEq => Some((4, None)),

                // Mantıksal
                Symbol::AndAnd => Some((3, Left)),
                Symbol::OrOr => Some((2, Left)),

                // Atama
                Symbol::Eq | Symbol::PlusEq | Symbol::MinusEq |
                Symbol::StarEq | Symbol::SlashEq | Symbol::PercentEq => Some((1, Right)),

                _ => Option::None,
            },

            Operation::Key(k) => match k {
                // `as` Rust'ta unary'den sonra, çarpmadan önce gelir
                Keyword::As => Some((12, Left)),
                // Return / Var: ifade başlatırlar, en düşük öncelik
                Keyword::Return | Keyword::Var => Some((0, Right)),
                _ => Option::None,
            },
        }
    }

    /// Operatörün (Sol, Sağ) parametre (operand) sayısını döner.
    /// Parametre almayan (noktalama, parantez) semboller (0, 0) döner.
    pub fn operand_count(&self) -> (u8, u8) {
        match self {
            // --- SYMBOL KONTROLLERİ ---
            Operation::Sym(symbol) => match symbol {
                // İkili (Binary) Operatörler: 1 sol, 1 sağ (Örn: a + b, x == y)
                Symbol::Plus | Symbol::Minus | Symbol::Star | Symbol::Slash | Symbol::Percent |
                Symbol::EqEq | Symbol::NotEq | Symbol::Lt | Symbol::LtEq | Symbol::Gt | Symbol::GtEq |
                Symbol::AndAnd | Symbol::OrOr |
                Symbol::Amp | Symbol::Pipe | Symbol::Caret | Symbol::Shl | Symbol::Shr |
                Symbol::Eq | Symbol::PlusEq | Symbol::MinusEq | Symbol::StarEq | Symbol::SlashEq | Symbol::PercentEq 
                => (1, 1),

                // Önek Unary (Prefix) Operatörler: 0 sol, 1 sağ (Örn: !x, ~y)
                Symbol::Bang | Symbol::Tilde => (0, 1),

                // Sonek Unary (Postfix) Operatörler: 1 sol, 0 sağ (Örn: Rust'taki x? kullanımı)
                Symbol::Question => (1, 0),

                // Özellik/Metod erişimi (Genelde 1 sol, 1 sağ parametre gibi düşünülür: obje.metod)
                Symbol::Dot | Symbol::Arrow => (1, 1),

                // Kalan tüm semboller (Parantezler, noktalı virgül vs.)
                _ => (0, 0),
            },

            // --- KEYWORD KONTROLLERİ ---
            Operation::Key(keyword) => match keyword {
                //Binary
                Keyword::As => (1, 1),

                //Prefix
                Keyword::Var |
                Keyword::Return
                 => (0, 1),
                

                //Postfix

                //bool and block


                // Let ve Const genelde kendi içlerinde bir statement (ifade) başlattığı için
                // klasik bir operatör gibi sol/sağ değerlendirmesine girmezler.
                _ => (0, 0),
            }
        }
    }
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