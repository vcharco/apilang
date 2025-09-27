pub mod errors;
pub mod lexer;
pub mod parser;
pub mod token;

use crate::transpiler::{
    errors::{CompilerError, ReadFileError},
    parser::core::parse,
};

use self::{lexer::Lexer, token::Token, token::TokenType};
use std::{fs, path::PathBuf};

pub fn process_file(file_path: &PathBuf) -> Result<(), Vec<CompilerError>> {
    let file_name = file_path.to_string_lossy().to_string();

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            let file_error = ReadFileError {
                file: file_name,
                error: format!("Failed to read file: {}", e),
            };
            return Err(vec![file_error.into()]);
        }
    };

    let mut lexer = Lexer::new(&content, Some(file_path.clone()));
    let mut tokens = Vec::new();
    let mut compiler_errors: Vec<CompilerError> = Vec::new();

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
            Err(lexer_errors) => {
                for error in lexer_errors {
                    compiler_errors.push(error.into());
                }
            }
        }
    }

    if let Err(e) = parse(
        tokens,
        &file_name,
        content.lines().map(|s| s.to_string()).collect(),
    ) {
        compiler_errors.push(e.into());
    }

    if !compiler_errors.is_empty() {
        return Err(compiler_errors);
    }

    Ok(())
}
