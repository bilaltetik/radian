use crate::lexer::types::*;
use crate::lexer::types::TokenKind::*;
use crate::lexer::utils::word;
use crate::parser::types::{AstNode};

/*
    create_blind_ast parserın 1. aşaması sembol tablosu ve komut görevlerinin uygulanması yok sadece şablonu çıkarıyor
    2. aşama şuanda kodlanmayacak kısım sembol tabloları ve görevlerin tasarlanması kısmı
*/

pub fn create_blind_ast(tokens: Vec<Token>) -> AstNode{
    let mut scope : Vec<usize> = vec![0];

    let mut counter:usize = 0;
    let mut program = AstNode::program();

    while !tokens.is_empty() && counter < tokens.len() {
        let current = &tokens[counter];
        match current.kind {
            // Operation
            TokenKind::Word(WordKind::Keyword(_)) | TokenKind::Symbol(_) => {
                let _operation = match current.kind {
                    TokenKind::Word(WordKind::Keyword(keyword)) => Operation::Key(keyword),
                    TokenKind::Symbol(symbol) => Operation::Sym(symbol),
                    _ => unreachable!(),
                };
                
                let (post ,pre) = _operation.operand_count();
                if(post == 0 && pre == 0){
                    //aslında operasyon değil sabit
                    let (precedence, assoc) = _operation.precedence().unwrap();
                }else{
                    //gerçek operasyon
                }

            }

            TokenKind::Number(_) | TokenKind::Literal(_) => {
                //operasyon değil sabit
            }

            _ => {

            }
        }
    }
    return program;
}
