use std::io::Result;

use crate::{lexer::core::tokenize, parser::core::parse};


mod lexer;
mod parser;

fn main() -> Result<()> {
    
    
    let my_code: &str = "var x = 5;var y = 0-5+x;";


    println!("The Code: {my_code}");
    let tokens = tokenize(my_code).unwrap() ;
    for token in &tokens{
       println!("Kind: {:?}, Value: {:?}, ({:?})",token.kind,token.lexeme, token.span);
    }
    println!();
    let program: parser::core::AstNode = parse(tokens);
    program.print_tree(0);

    
    /*
    for code_str in my_code{
        println!("The Code: {code_str}");
        for token in tokenize(code_str).unwrap() {
            println!("Kind: {:?}, Value: {:?}, ({:?})",token.kind,token.lexeme, token.span);
        }
        println!();

    }
    */

    

    Ok(())
}