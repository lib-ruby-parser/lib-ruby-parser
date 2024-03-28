use std::ffi::OsString;

#[derive(Debug)]
pub(crate) enum InputToParse {
    Eval(OsString),
    Glob(OsString),
}

impl InputToParse {
    pub(crate) fn into_files(self) -> Vec<InputFile> {
        match self {
            InputToParse::Eval(code) => vec![InputFile::eval(code)],
            InputToParse::Glob(pattern) => InputFile::glob(pattern),
        }
    }
}

#[derive(Clone)]
pub(crate) struct InputFile {
    pub(crate) filepath: String,
    pub(crate) code: Vec<u8>,
}

impl InputFile {
    fn eval(code: OsString) -> Self {
        Self {
            filepath: String::from("(eval)"),
            code: code.into_encoded_bytes(),
        }
    }

    fn glob(pattern: OsString) -> Vec<Self> {
        glob::glob(pattern.to_str().unwrap())
            .expect("invalid glob pattern")
            .map(|f| f.unwrap().to_str().unwrap().to_string())
            .map(|filepath| Self {
                code: std::fs::read(&filepath).unwrap(),
                filepath,
            })
            .collect()
    }
}
