// Radian Lang - lexer çekirdeği
//
// Buraya gelecekler:
//   - struct Lexer { ... }        (kaynak, konum, line/col)
//   - peek() / advance()          (line/col sadece advance içinde güncellenir)
//   - next_token() -> Result<Token, LexError>
//         ilk karaktere bakıp dağıtır:
//           rakam        -> utils::number::lex_number
//           harf, '_'    -> lex_word
//           '"' , '\''   -> utils::string::lex_string
//           diğer        -> utils::symbol::lex_symbol
//         boşluk ve yorum atlanır, token üretmez
//   - tokenize(input) -> Result<Vec<Token>, LexError>

use std::result;
use std::{collections::hash_map, iter::Map};

use crate::lexer::types::*;
use crate::lexer::utils::*;

pub fn peek(s: &str, pos: usize) -> Option<char>{
    s[pos..].chars().next()
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
	let _ = input;
    let mut tokens: Vec<Token> = Vec::new();
    let mut span= Span{line:0, col:0};
    let mut pos = 0;
    
    //token üretme döngüsü
    while !input.is_empty() {
        let c = peek(input, pos);   // her iterasyonda tek peek

        match c {
            //eof
            None => {
                tokens.push( Token { kind: TokenKind::Eof, lexeme: String::from(""), span } );
                break;
            }

            //whitespace
            Some('\n') => {
                span.line += 1; span.col = 0; pos += 1;
            }
            Some(' ') => {
                span.col += 1; pos += c.unwrap().len_utf8();
            }
            Some('\t') => {
                span.col += 5; pos += c.unwrap().len_utf8();
            }

            //Comments

            //tokens
            Some(c) if match_number(c) => { 
                let result = lex_number(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Number(result.0), lexeme: input[pos..(result.1)].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
             }
            Some(c) if match_word(c)   => {
                let result = lex_word(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Word(result.0), lexeme: input[pos..(result.1)].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
            }
            Some(c) if match_symbol(c).is_some()   => {
                let result = lex_symbol(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Symbol(result.0), lexeme: input[pos..(result.1)].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
            }
            Some(c) if match_string(c).is_some() => { 
                let result = lex_string(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Literal(result.0), lexeme: input[pos..(result.1)].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
             }

            //error
            Some(_) => {
                return Err(format!(
                    "Unexpected character at ({},{})",
                    span.line, span.col
                ));
            }
        }
    }

	Ok((tokens))
}