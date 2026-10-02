use crate::lexer::types::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    None,
    SymbolEntryKey(String),
    Token(Token),
}

#[derive(Debug, Clone)]
pub enum SymbolEntry {
    Variable { kind: VariableTypes },
    Function {
        domain: Box<VariableTypes>,
        range: Box<VariableTypes>,
    },
}

#[derive(Debug, Clone)]
pub enum Command {
    Null,
    Program,
    Statement,
    ValueAs,
    TypeAs,
    CreateVar,
    DirectExpression,
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub cmd: Command,
    pub args: Vec<AstNode>,
    pub value: Expression,
}

impl AstNode {
    pub fn program() -> AstNode {
        AstNode {
            cmd: Command::Program,
            args: Vec::new(),
            value: Expression::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableTypes {
    None,
    Null,
    I8,
    I16,
    I32,
    I64,
    Tuple(Vec<VariableTypes>),
}