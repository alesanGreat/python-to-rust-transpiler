use regex::Regex;
use crate::types::{Token, TokenType};

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    
    let re_number = Regex::new(r"^[0-9]+").unwrap();
    let re_identifier = Regex::new(r"^[[:alpha:]_][[:alpha:][:digit:]_]*").unwrap();
    let re_string = Regex::new(r#"^"([^"\\]*(\\.[^"\\]*)*)""#).unwrap();
    let re_symbol = Regex::new(r"^(\(|\)|,|:|#|\+|\-|\*|\/\/|\/|=|\.|>|<)").unwrap();

    let mut line_number = 1;
    let mut column = 1;
    
    for line in code.lines() {
        let mut pos = 0;
        let line_chars: Vec<char> = line.chars().collect();

        while pos < line_chars.len() {
            let rest: String = line_chars[pos..].iter().collect();

            // Manejar comentarios
            if rest.starts_with("#") {
                let comment = rest;
                tokens.push(Token {
                    token_type: TokenType::Comment,
                    value: comment,
                });
                break;
            }
            
            // Keywords
            else if rest.starts_with("sdl2.") {
                 tokens.push(Token {
                    token_type: TokenType::Keyword,
                    value: "sdl2.".to_string(),
                });
                pos += "sdl2.".len();
                column += "sdl2.".len();
             }
            // Otros keywords (while, if, etc.)
             else if is_keyword(&rest, "while", pos, &line_chars) {
                add_keyword_token(&mut tokens, "while", &mut pos, &mut column);
            }
            else if is_keyword(&rest, "if", pos, &line_chars) {
                add_keyword_token(&mut tokens, "if", &mut pos, &mut column);
            }
            else if is_keyword(&rest, "true", pos, &line_chars) {
                add_keyword_token(&mut tokens, "true", &mut pos, &mut column);
            }
            else if is_keyword(&rest, "false", pos, &line_chars) {
                 add_keyword_token(&mut tokens, "false", &mut pos, &mut column);
             }
            
            // Identificadores
            else if let Some(mat) = re_identifier.find(&rest) {
                let identifier = mat.as_str().to_string();
                tokens.push(Token {
                    token_type: TokenType::Identifier,
                    value: identifier,
                });
                pos += mat.end();
                column += mat.end();
            }
            
            // Números
            else if let Some(mat) = re_number.find(&rest) {
                tokens.push(Token {
                    token_type: TokenType::Number,
                    value: mat.as_str().to_string(),
                });
                pos += mat.end();
                column += mat.end();
            }
            
            // Strings
            else if let Some(mat) = re_string.find(&rest) {
                tokens.push(Token {
                    token_type: TokenType::String,
                    value: mat.as_str().to_string(),
                });
                pos += mat.end();
                column += mat.end();
            }
            
            // Símbolos
            else if let Some(mat) = re_symbol.find(&rest) {
                tokens.push(Token {
                    token_type: TokenType::Symbol,
                    value: mat.as_str().to_string(),
                });
                pos += mat.end();
                column += mat.end();
            }
            
            // Espacios en blanco
            else if line_chars[pos].is_whitespace() {
                pos += 1;
                column += 1;
            }
            
            // Carácter no reconocido
            else {
                println!("Warning: Carácter no reconocido '{}' en línea {}, columna {}", 
                    line_chars[pos], line_number, column);
                pos += 1;
                column += 1;
            }
        }
        
        tokens.push(Token {
            token_type: TokenType::Eol,
            value: "\n".to_string(),
        });
        
        line_number += 1;
        column = 1;
    }
    
    tokens
}

fn is_keyword(rest: &str, keyword: &str, pos: usize, line_chars: &[char]) -> bool {
    rest.starts_with(keyword) && 
        (pos + keyword.len() >= line_chars.len() || 
         line_chars[pos + keyword.len()].is_whitespace() || 
         !line_chars[pos + keyword.len()].is_alphanumeric())
}

fn add_keyword_token(tokens: &mut Vec<Token>, keyword: &str, pos: &mut usize, column: &mut usize) {
    tokens.push(Token {
        token_type: TokenType::Keyword,
        value: keyword.to_string(),
    });
    *pos += keyword.len();
    *column += keyword.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let code = "x = 10\n";
        let tokens = tokenize(code);
        assert_eq!(tokens.len(), 4); // Identifier, Symbol(=), Number, EOL
        assert_eq!(tokens[0].value, "x");
        assert_eq!(tokens[1].value, "=");
        assert_eq!(tokens[2].value, "10");
    }

    #[test]
    fn test_sdl2_keyword() {
         let code = "sdl2.init()\n";
         let tokens = tokenize(code);
         assert!(tokens.iter().any(|t| t.value == "sdl2."));
     }
}