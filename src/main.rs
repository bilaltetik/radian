use std::io::Result;

use crate::lexer::core::tokenize;


mod lexer;
mod parser;

fn main() -> Result<()> {
    
    
    let my_code: &str = "var x = 5;";


    println!("The Code: {my_code}");
    let tokens = tokenize(my_code).unwrap() ;
    for token in tokens{
       println!("Kind: {:?}, Value: {:?}, ({:?})",token.kind,token.lexeme, token.span);
    }
    println!();

    

    
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