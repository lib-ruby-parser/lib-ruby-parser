use std::fs;

pub fn files_under_dir(dir: &str) -> Vec<String> {
    fs::read_dir(dir)
        .expect(&format!("{} doesn't exist", dir))
        .map(|res| res.expect("failed to get dir items").path())
        .map(|path| path.to_str().expect("file has non-utf8 path").to_owned())
        .collect::<Vec<_>>()
}
