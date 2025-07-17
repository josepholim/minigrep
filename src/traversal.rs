use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_files(base: &Path, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // If not recursive or stdin, return directly
    if !recursive || base.to_string_lossy() == "-" {
        files.push(base.to_path_buf());
        return Ok(files);
    }
    
    for entry in WalkDir::new(base)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        files.push(entry.path().to_path_buf());
    }

    Ok(files)
}