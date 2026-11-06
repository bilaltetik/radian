use crate::{lexer::types::*, parser::types::*};

pub(super) fn slot_of(kind: &AstKind) -> Slot {
    match kind {
        AstKind::Expression(_) => Slot::EXPR,
        AstKind::Statment      => Slot::STMT,
        AstKind::Operand       => Slot::OP,
        AstKind::Marker(_)     => Slot::Marker,
        AstKind::Program       => Slot::NONE,
    }
}
pub(super) fn marker_of(kind: &AstKind) -> Markers {
    match kind {
        AstKind::Marker(m) => *m,
        _ => Markers::NONE,
    }
}

impl AstNode {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        
        if self.args.is_empty() {
            println!("{}AST({:?}, {:?}) {{}}", tabs, self.kind, self.token.lexeme);
        } else {
            println!("{}AST({:?}, {:?}) {{", tabs, self.kind, self.token.lexeme);
            for arg in &self.args {
                arg.print_tree(depth + 1);
            }
            println!("{}}}", tabs);
        }
    }

    pub fn program(tokens: Vec<Token>) -> AstNode {
        // Vektörün boş olma durumunu yönetmek için unwrap yerine pattern matching/if let kullanmanız önerilir.
        // Clone veya Copy ile referans bağımlılığını koparıyoruz.
        let last_token = tokens.last().unwrap().clone(); 
        
        let mut args: Vec<AstNode> = Vec::new();
        
        // Değişken ismini 't' yaptık ki dışarıdakini ezmesin
        for t in tokens {
            match t.kind {
                TokenKind::TokenNumber(_) | TokenKind::TokenLiteral(_) => {
                    args.push(AstNode { token: t, kind: AstKind::Expression(ExpressionKind::Value), args: Vec::new() });
                }
                TokenKind::TokenWord(WordKind::Identifier) => {
                    args.push(AstNode { token: t, kind: AstKind::Expression(ExpressionKind::Identifier), args: Vec::new() });
                }
                TokenKind::TokenSymbol(_) | TokenKind::TokenWord(WordKind::Keyword(_)) => {
                    args.push(AstNode { token: t, kind: AstKind::Operand, args: Vec::new() });
                }
                TokenKind::Eof => {
                    break;
                }
            }
        }

        AstNode { token: last_token, kind: AstKind::Program, args }
    }
}
