extern crate glob;
use glob::glob;

#[allow(dead_code)]
pub fn each_ruby_file<F>(path: &str, cb: &F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&str) -> Result<(), Box<dyn std::error::Error>>,
{
    for entry in glob(path).expect("invalid glob pattern") {
        let entry = entry?;
        cb(entry.to_str().unwrap())?
    }

    Ok(())
}
