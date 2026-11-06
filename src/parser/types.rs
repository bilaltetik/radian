use crate::lexer::types::*;

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

    pub struct Slot: u8 {
        const EXPR    = 1 << 0;  // değer üreten herşey: Atom, Infix, Prefix, Postfix
        const STMT    = 1 << 1;  // Statement
        const OP      = 1 << 2;  // çıplak operatör (örn: blok içinde pattern)
        const Marker  = 1 << 3;
        const Group   = 1 << 4;
        const NONE    = 0b1000;  // boş / yok
    }
}

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

    pub struct Markers: u8 {
        const LeftBracket        = 1 << 0; // "("
        const LeftHardBracket    = 1 << 1; // "["
        const LeftSoftBracket    = 1 << 2; // "{"
        const NONE    = 0b1000;  // boş / yok
    }
}


pub enum Operation{
    Sym(SymbolDomain),
    Key(KeywordDomain),
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc{
    Left,
    Right,
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCmd{
    PlusSum,    //5+3
    PlusSign,   // +3
    MinusSub,   //5-3
    MinusSign,  // -8
    ValueAs,    //x = 5
    TypeAs,     //x as int
    Create,     // var x;
    FinishStatment, // ;

    LeftMathBracket,    // 3*(5+...
    RightMathBracket,   // ...+3)

    RightTupleBracket,  //(a,b,c)

    ExpGroup,   // a,b,c
    None,   //Kürdistan
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind{
    Value,
    Identifier,
    Block,
    Group,
    Complex
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstKind{
    Program,
    Statment,
    Expression(ExpressionKind),
    Operand,
    Marker(Markers)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstNode{
    pub(super) token:Token,
    pub(super) kind:AstKind,
    pub(super) args: Vec<AstNode>,
}
