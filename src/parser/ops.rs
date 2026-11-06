use std::vec;

use crate::{lexer::types::*, parser::types::{ ExpressionKind::Identifier, OpCmd::{ TypeAs}, *} };

impl From<OpCmd> for (Vec<Slot>, Vec<Slot>) {
    fn from(value: OpCmd) -> Self {
        match value {
            // Binary
            OpCmd::PlusSum | OpCmd::MinusSub | OpCmd::ValueAs | OpCmd::TypeAs | OpCmd::ExpGroup =>
            { (vec![Slot::EXPR | Slot::Marker], vec![Slot::EXPR ]) },

            // Unary
            OpCmd::PlusSign | OpCmd::MinusSign |
            OpCmd::Create => 
            { (vec![], vec![Slot::EXPR]) },

            // Statement Terminator (Noktalı Virgül)
            OpCmd::FinishStatment => 
            { (vec![Slot::EXPR | Slot::STMT], vec![]) },

            OpCmd::LeftMathBracket => { (vec![], vec![]) },
            OpCmd::RightMathBracket => { (vec![Slot::Marker, Slot::EXPR], vec![]) }

            OpCmd::RightTupleBracket => { (vec![Slot::Marker, Slot::Group], vec![]) }

            OpCmd::None => { (vec![], vec![]) }
        }
    }
}

impl From<OpCmd> for (Vec<Markers>, Vec<Markers>) {
    fn from(value: OpCmd) -> Self {
        match value {
            // RightMathBracket: Solunda 'LeftBracket' (yani "(" ) bekler
            OpCmd::RightMathBracket => (
                vec![Markers::LeftBracket, Markers::NONE], 
                vec![]
            ),

            // İleride RightTupleBracket eklersen:
            OpCmd::RightTupleBracket => (
                vec![Markers::LeftHardBracket, Markers::NONE], 
                vec![]
            ),
            
            // Diğer operatörler marker beklemediği için NONE döner
            _ => (vec![Markers::NONE; 2], vec![Markers::NONE; 2]) // Kendi boyutlarına göre ayarla
        }
    }
}

impl From<SymbolDomain> for Operation {
    fn from(op: SymbolDomain) -> Operation{
        Operation::Sym(op)
    }
}

impl From<KeywordDomain> for Operation {
    fn from(op: KeywordDomain) -> Operation{
        Operation::Key(op)
    }
}

impl Operation {


    pub(super) fn from_token_kind(kind: TokenKind) -> (Operation, bool){
        match kind {
            TokenKind::TokenWord(WordKind::Keyword(keyword)) => {
                (Operation::Key(keyword), true)
            }
            TokenKind::TokenSymbol(symbol) => (Operation::Sym(symbol), true),
            _ => (Operation::Invalid, false),
        }
    }
}

impl From<SymbolDomain> for Vec<OpCmd>{
    fn from(op: SymbolDomain) -> Vec<OpCmd>{
        match op {
            SymbolDomain::Plus => {vec![OpCmd::PlusSign,OpCmd::PlusSum]}
            SymbolDomain::Minus => {vec![OpCmd::MinusSign,OpCmd::MinusSub]}
            SymbolDomain::Eq => {vec![OpCmd::ValueAs]}
            SymbolDomain::Semicolon => {vec![OpCmd::FinishStatment]}
            SymbolDomain::LeftBracket => {vec![OpCmd::LeftMathBracket]}
            SymbolDomain::RightBracket => {vec![OpCmd::RightMathBracket, OpCmd::RightTupleBracket]}
            SymbolDomain::Comma => {vec![OpCmd::ExpGroup]}
        }
    }
}

impl From<KeywordDomain> for Vec<OpCmd>{
    fn from(op: KeywordDomain) -> Vec<OpCmd>{
        match op {
            KeywordDomain::As => {vec![TypeAs]},
            KeywordDomain::Var => {vec![OpCmd::Create]}
        }
    }
}

impl From<Operation> for Vec<OpCmd>{
    fn from(op: Operation) -> Vec<OpCmd>{
        match op {
            Operation::Sym(sym) => Vec::<OpCmd>::from(sym),
            Operation::Key(key) => Vec::<OpCmd>::from(key),
            Operation::Invalid => vec![],
        }
    }
}

impl OpCmd {
    pub fn precedence(self) -> u8 {
        match self {
            OpCmd::FinishStatment => 0,
            OpCmd::Create         => 2,

            OpCmd::ValueAs        => 3,    // assoc: Right

            OpCmd::ExpGroup       => 5,
            OpCmd::PlusSum        => 10,
            OpCmd::MinusSub       => 10,
            OpCmd::TypeAs         => 11,
            OpCmd::PlusSign       => 12,
            OpCmd::MinusSign      => 12,

            OpCmd::LeftMathBracket => 250,
            OpCmd::RightTupleBracket => 254,
            OpCmd::RightMathBracket => 254,

            OpCmd::None           => 255,
        }
    }

    pub fn assoc(self) -> Assoc {
        match self {
            OpCmd::PlusSum   => Assoc::Left,
            OpCmd::MinusSub  => Assoc::Left,

            OpCmd::PlusSign  => Assoc::Right,
            OpCmd::MinusSign => Assoc::Right,

            OpCmd::ValueAs   => Assoc::Right,
            OpCmd::TypeAs    => Assoc::Left,

            OpCmd::Create    => Assoc::Left,

            OpCmd::FinishStatment => Assoc::Left,

            OpCmd::LeftMathBracket => Assoc::Left,
            OpCmd::RightMathBracket => Assoc::Left,
            OpCmd::RightTupleBracket => Assoc::Left,

            OpCmd::ExpGroup         => Assoc::Left,

            OpCmd::None      => Assoc::None,

        }
    }

    pub fn result_kind(self) -> Option<AstKind> {
        match self {
            //Expr
            OpCmd::PlusSum | OpCmd::MinusSub | OpCmd::PlusSign | OpCmd::MinusSign
            => {
                Some(AstKind::Expression(ExpressionKind::Complex))
            }
            //Identifier
            OpCmd::TypeAs | OpCmd::ValueAs | OpCmd::Create
            => {
                Some(AstKind::Expression(Identifier))
            }

            OpCmd::LeftMathBracket => {
                Some(AstKind::Marker(Markers::LeftBracket))
            }

            OpCmd::RightMathBracket => {
                Some(AstKind::Expression(ExpressionKind::Complex))
            }

            OpCmd::RightTupleBracket => {
                Some(AstKind::Expression(ExpressionKind::Group))
            }

            //Statment
            OpCmd::FinishStatment => {
                Some(AstKind::Statment)
            }
            OpCmd::ExpGroup => Some(AstKind::Expression(ExpressionKind::Group)),

            OpCmd::None => {Option::None} //standart durumda Program işlenmez
        }
    }
}
