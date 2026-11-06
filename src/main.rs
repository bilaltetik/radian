use std::io::Result;

use crate::{lexer::core::tokenize, parser::core::parse};
use comfy_table::{Table, ContentArrangement};

mod lexer;
mod parser;

fn main() -> Result<()> {
    let mut table = Table::new();
    table.set_header(vec!["Kind", "Value", "Line", "Col"]);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    
    let my_code: &str = "var x = (1,2);";

    //a
    
    println!("The Code:\n{my_code}\n");
    let tokens = tokenize(my_code).unwrap() ;
    
    for token in &tokens {
        table.add_row(vec![
            format!("{:?}", token.kind), 
            token.lexeme.clone(),
            token.span.line.to_string(),
            token.span.col.to_string(),
        ]);
    }

    println!("{table}");

    println!();
    let program: parser::types::AstNode = parse(tokens);
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