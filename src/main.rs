use apilang::transpiler::lexer::LexerError;
use apilang::transpiler::process_file;
use apilang::transpiler::token::Token;
use apilang::util::file::find_api_files;
use rayon::prelude::*;
use std::path::PathBuf;

fn main() {
    let current_dir = PathBuf::from(".");
    let files: Vec<PathBuf> = find_api_files(&current_dir);

    let results: Result<Vec<Vec<Token>>, Vec<LexerError>> = files
        .par_iter()
        .map(|file_path| process_file(file_path))
        .collect();

    match results {
        Ok(all_tokens) => {
            let flattened_tokens: Vec<Token> = all_tokens.into_iter().flatten().collect();
            println!("Tokens recolectados: {}", flattened_tokens.len());
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }
}
