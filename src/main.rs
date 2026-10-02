use std::io::Result;

use crate::lexer::core::tokenize;


mod lexer;

fn main() -> Result<()> {
    
    let my_code = "
        x = 5;
        int main(){
            print(x);
            hesapla(lokok);
            +=1
        }
    ".to_string();

    let tokens = tokenize(&my_code).unwrap();

    for token in tokens {
        //println!("{:?}", token);
        println!("{:?} - {:?}", token.kind, token.lexeme);
    }

    Ok(())
}