use std::io;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;

pub fn collect_files(base: &Path, recursive: bool, ignore_file: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // If not recursive or stdin, return directly
    if !recursive || base.to_string_lossy() == "-" {
        files.push(base.to_path_buf());
        return Ok(files);
    }

    let mut builder = WalkBuilder::new(base);
    builder.add_ignore(ignore_file);
    for result in builder.build() {
        let dent = result
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if dent.file_type().map_or(false, |ft| ft.is_file()) {
            files.push(dent.into_path());
        }
    }

    Ok(files)
}
