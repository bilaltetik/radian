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
        let c = peek(input, pos);

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
            Some('/') if input[pos..].starts_with("//") => {
                let len = input[pos..].find('\n').unwrap_or(input.len() - pos);
                span.col += input[pos..pos + len].chars().count();
                pos += len;
                continue;
            }

            Some('/') if input[pos..].starts_with("/*") => {
                let end = match input[pos + 2..].find("*/") {
                    Some(i) => pos + 2 + i + 2,
                    None => {
                        return Err(format!(
                            "Unterminated block comment at ({},{})",
                            span.line, span.col
                        ));
                    }
                };

                for ch in input[pos..end].chars() {
                    if ch == '\n' { span.line += 1; span.col = 0; } else { span.col += 1; }
                }
                pos = end;
                continue;
            }



            //tokens
            Some(c) if match_number(c) => { 
                let result = lex_number(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Number(result.0), lexeme: input[pos..pos + result.1].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
             }
            Some(c) if match_word(c)   => {
                let result = lex_word(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Word(result.0), lexeme: input[pos..pos + result.1].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
            }
            Some(c) if match_symbol(c).is_some()   => {
                let result = lex_symbol(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Symbol(result.0), lexeme: input[pos..pos + result.1].to_string(), span });
                span.col += result.1;
                pos += result.1;
                continue;
            }
            Some(c) if match_string(c).is_some() => { 
                let result = lex_string(&input[pos..]).unwrap();
                tokens.push(Token { kind: TokenKind::Literal(result.0), lexeme: input[pos..pos + result.1].to_string(), span });
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

	Ok(tokens)
}