pub mod lexer;
pub mod token;

use self::{lexer::Lexer, lexer::LexerError, token::Token, token::TokenType};
use std::{fs, path::PathBuf};

pub fn process_file(file_path: &PathBuf) -> Result<Vec<Token>, Vec<LexerError>> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            let file_error = LexerError {
                line: 0,
                file: file_path.to_string_lossy().to_string(),
                error: format!("Failed to read file: {}", e),
                line_content: String::new(),
                char_index: 0,
            };
            return Err(vec![file_error]);
        }
    };

    let mut lexer = Lexer::new(&content, Some(file_path.clone()));
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(Some(token)) => {
                if token.token_type == TokenType::Eof {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Ok(None) => {
                break;
            }
            Err(mut lexer_errors) => {
                errors.append(&mut lexer_errors);
            }
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(tokens)
    }
}
