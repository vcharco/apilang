use apilang::transpiler::errors::CompilerError;
use apilang::transpiler::process_file;
use apilang::util::file::find_api_files;
use rayon::prelude::*;
use std::path::PathBuf;

fn main() {
    let current_dir = PathBuf::from(".");
    let files: Vec<PathBuf> = find_api_files(&current_dir);

    let results: Result<Vec<()>, Vec<CompilerError>> = files
        .par_iter()
        .map(|file_path| process_file(file_path))
        .collect();

    match results {
        Ok(_) => {
            println!("Everything went fine!")
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }
}
