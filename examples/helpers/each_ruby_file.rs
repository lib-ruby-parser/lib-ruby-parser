use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn each_ruby_file<F>(path: &Path, cb: &F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&str) -> Result<(), Box<dyn std::error::Error>>,
{
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            each_ruby_file(&path, cb)?;
        }
    } else if path.extension().map(|s| s.to_str()).flatten() == Some("rb") {
        let path = path.to_str().ok_or("Invalid path")?;
        cb(path)?;
    }
    Ok(())
}
