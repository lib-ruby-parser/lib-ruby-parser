use std::fs;
use std::io;
use std::path::Path;

#[allow(dead_code)]
pub fn each_ruby_file(path: &Path, cb: &dyn Fn(&str)) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                each_ruby_file(&path, cb)?;
            } else if path.extension().is_some() && path.extension().unwrap() == "rb" {
                cb(&entry.path().to_str().unwrap());
            }
        }
    } else if path.extension().unwrap() == "rb" {
        cb(&path.to_str().unwrap())
    }
    Ok(())
}
