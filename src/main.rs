// main.rs
mod lexer;
mod token;

use apilang::util::file::find_api_files;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

use lexer::Lexer;
use token::TokenType;

fn main() {
    let current_dir = PathBuf::from(".");
    let files: Vec<PathBuf> = find_api_files(&current_dir);

    println!(
        "Found {} .api files. Processing in parallel...",
        files.len()
    );

    // Procesa los archivos en paralelo usando Rayon
    files.par_iter().for_each(|file_path| {
        process_file(file_path);
    });

    println!("\nAll files processed successfully.");
}

fn process_file(file_path: &PathBuf) {
    println!("\nProcessing file: {:?}", file_path);

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", file_path, e);
            return;
        }
    };

    // Pasa la ruta del archivo a la nueva función de 'new'
    let mut lexer = Lexer::new(&content, Some(file_path.clone()));
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token.unwrap().token_type == TokenType::Eof {
            break;
        }
    }

    println!("Tokens for {:?}:", file_path);
    for token in tokens {
        println!("{:?}", token);
    }
}
