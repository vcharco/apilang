use std::path::PathBuf;

use walkdir::WalkDir;

pub fn find_api_files(dir: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |s| s.eq_ignore_ascii_case("api"))
        })
        .map(|entry| entry.path().to_owned())
        .collect()
}
