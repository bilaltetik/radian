use crate::lexer::types::*;
use crate::lexer::types::TokenKind::*;
use crate::lexer::utils::word;
use crate::parser::types::{AstNode};

pub fn create_blind_ast(tokens: Vec<Token>) -> AstNode{
    let mut scope : Vec<usize> = vec![0];

    let mut counter:usize = 0;
    let mut program = AstNode::program();

    while !tokens.is_empty() {
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






/*
    Değişken üretme
    var x;
    semicolon(
        create(x) -> [x, null];
    ) hata verecek yada sonraki ilk kullanım belirleyecek
*/



/*
    var x as T;
    semicolon(
        typeAs(
            create(c) -> [x, null],
            T
        ) -> [x, T]
    ) //x T tipinde üretildi
    C{
        T x;
    }

    var x = 5;
    semicolon(
        valueAs(
            create(x) -> [x, null],
            [5, null]
        ) -> [x, i32]   //eğer ikisi de null ise 5'in varsayılan tipi,
                        //eğer x belliyse x in tipi, eğer 5 belliyse 5 in tipi 
                        //eğer ikisi de belliyse ve aynıysa değişme farklıysa hata ver
    ) //x üretildi içine 5 değeri atıldı ve tipi 5 in varsayılan tipi
    C{
        int32_t x = 5; 
    }

    //"as", "=" den öncelikli
    var x as T = a;
    semicolon(
        valueAs(
            typeAs(
                create(x) -> [x, null],
                T
            ) -> [x, T],
            [a, null]
        ) // x değişkeninin değeri a ve tipi T
    )
    C{
        T x = a;
    }

    var x = 5 as T;
    semicolon(
        valueAs(
            create(x) -> [x, null],
            typeAs(5, T) -> [5, T],
        ) -> [x, T];
    )


    //değer değiştirme
    x = 5;
    semicolon(
        valueAs(
            [x, T];
            [5, null];
        ) //eğer x yoksa hata ver varsa değişken değiştirme 2. deki adımları izle
    )
*/

